#!/usr/bin/env bash
# Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
# Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
#
# Fetches the BMF income tax calculator (Einkommensteuerrechner) for each case in a points list and
# appends the results to tests/fixtures/conformance/<points-name>.csv. Cases already in the fixture are skipped,
# so an interrupted run resumes where it stopped. Raw result pages are kept under CONFORMANCE_RAW_DIR.
#
# Usage: fetch.sh scripts/conformance/points/<year>.csv
set -euo pipefail

if [ "$#" -ne 1 ]; then
  echo "usage: $0 scripts/conformance/points/<year>.csv" >&2
  exit 2
fi

points="$1"
if [ ! -f "$points" ]; then
  echo "missing points file: $points" >&2
  exit 2
fi

root="$(cd "$(dirname "$0")/../.." && pwd)"
stem="$(basename "$points" .csv)"
fixtures_dir="${CONFORMANCE_FIXTURE_DIR:-$root/tests/fixtures/conformance}"
fixture="$fixtures_dir/$stem.csv"
runs="$fixtures_dir/runs.csv"
raw_root="${CONFORMANCE_RAW_DIR:-${TMPDIR:-/tmp}/conformance-raw}"
form_url="https://www.bmf-steuerrechner.de/ekst/eingabeformekst.xhtml"
pause="${CONFORMANCE_PAUSE_SECONDS:-1}"
refresh_every="${CONFORMANCE_REFRESH_EVERY:-50}"
attempts="${CONFORMANCE_ATTEMPTS:-3}"

work="$(mktemp -d "${TMPDIR:-/tmp}/conformance-fetch.XXXXXX")"
trap 'rm -rf "$work"' EXIT
jar="$work/cookies.txt"

german_cents() {
  local digits
  digits="$(printf '%s' "$1" | tr -cd '0-9,')"
  if [[ ! "$digits" =~ ^[0-9]+,[0-9]{2}$ ]]; then
    return 1
  fi
  digits="${digits/,/}"
  echo $((10#$digits))
}

cents_to_german() {
  printf '%d,%02d' $(($1 / 100)) $(($1 % 100))
}

new_session() {
  rm -f "$jar"
  local code
  code="$(curl -sS -c "$jar" -b "$jar" -o "$work/form.html" -w '%{http_code}' "$form_url")"
  if [ "$code" != 200 ]; then
    echo "form returned HTTP $code" >&2
    return 1
  fi
  viewstate="$(htmlq -a value 'form#bmf_form_ekst input[name="javax.faces.ViewState"]' -f "$work/form.html")"
  if [ -z "$viewstate" ]; then
    echo "no ViewState on the form" >&2
    return 1
  fi
}

query() {
  local year="$1" filing="$2" cents="$3" raw="$4" pv=false zve code echoed total
  if [ "$filing" = joint ]; then
    pv=true
  fi
  zve="$(cents_to_german "$cents")"
  code="$(curl -sS -L -b "$jar" -c "$jar" -X POST "$form_url" \
    --data-urlencode "bmf_form_ekst=bmf_form_ekst" \
    --data-urlencode "bmf_form_ekst:ekst_zve=$zve" \
    --data-urlencode "bmf_form_ekst:ekst_pv=$pv" \
    --data-urlencode "bmf_form_ekst:ekst_bj=$year" \
    --data-urlencode "bmf_form_ekst:income_ekst=Berechnen" \
    --data-urlencode "javax.faces.ViewState=$viewstate" \
    -o "$raw" -w '%{http_code}' </dev/null)"
  if [ "$code" != 200 ]; then
    echo "calculator returned HTTP $code" >&2
    return 1
  fi
  echoed="$(htmlq -t '#ekst_result_zve' -f "$raw")"
  if [ -z "$echoed" ]; then
    echo "no result page (session lost or error page)" >&2
    return 1
  fi
  if [ "$(german_cents "$echoed")" != "$cents" ]; then
    echo "echoed zvE $echoed does not match $zve" >&2
    return 1
  fi
  # The Einkommensteuer row, not the Summe row: the Summe also includes the Solidaritätszuschlag.
  total="$(htmlq -t 'td[headers="ekst_best_table_r_ekst ekst_best_table_betrag"]' -f "$raw")"
  german_cents "$total"
}

header="case_id,year_option,filing_status,taxable_income_cents,income_tax_cents,role,run_id"
mkdir -p "$(dirname "$fixture")"
if [ ! -f "$fixture" ]; then
  echo "$header" > "$fixture"
fi
if [ ! -f "$runs" ]; then
  echo "run_id,papversion,fetched_at,source" > "$runs"
fi

done_ids="$work/done.txt"
awk -F, 'NR > 1 { print $1 }' "$fixture" > "$done_ids"
pending="$work/pending.csv"
awk -F, -v done_file="$done_ids" 'FILENAME == done_file { done[$1] = 1; next } FNR > 1 && !($1 in done)' "$done_ids" "$points" > "$pending"

count="$(wc -l < "$pending" | tr -d ' ')"
if [ "$count" -eq 0 ]; then
  echo "nothing to fetch: $fixture already has every case in $points"
  exit 0
fi

new_session
papversion="$(htmlq -a src 'script[src^="/papversion/"]' -f "$work/form.html" | sed -E 's#.*papversion_([0-9.]+)\.js.*#\1#')"
run_id="run-$(date -u +%Y%m%dT%H%M%SZ)"
printf '%s,%s,%s,%s\n' "$run_id" "$papversion" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" bmf_ekst_form >> "$runs"
raw_dir="$raw_root/$run_id"
mkdir -p "$raw_dir"
echo "$run_id: fetching $count cases from $points (papversion $papversion)"

i=0
while IFS=, read -r case_id year filing cents role; do
  i=$((i + 1))
  if [ $((i % refresh_every)) -eq 0 ]; then
    new_session || true
  fi
  raw="$raw_dir/$case_id.html"
  ok=0
  tax=""
  for attempt in $(seq 1 "$attempts"); do
    if tax="$(query "$year" "$filing" "$cents" "$raw")"; then
      ok=1
      break
    fi
    echo "retry $case_id (attempt $attempt of $attempts)" >&2
    sleep $((attempt * 5))
    new_session || true
  done
  if [ "$ok" -ne 1 ]; then
    echo "failed $case_id; rerun to resume" >&2
    exit 1
  fi
  printf '%s,%s,%s,%s,%s,%s,%s\n' "$case_id" "$year" "$filing" "$cents" "$tax" "$role" "$run_id" >> "$fixture"
  echo "ok $case_id $tax"
  sleep "$pause"
done < "$pending"

echo "$run_id: wrote $count cases to $fixture"
