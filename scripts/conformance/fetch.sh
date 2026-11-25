#!/usr/bin/env bash
# Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
# Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
#
# Fetches the BMF income tax calculator (Einkommensteuerrechner) for each case on stdin, and appends the results
# to tests/fixtures/conformance/<year_option>.csv. A case is one line: year_option,filing_status,taxable_income_cents,role.
# scripts/conformance/points.rs writes them. A case whose filing status and income the fixture already has is
# skipped, so a rerun resumes where it stopped. With --dry-run nothing is written: each case is printed with the
# calculator's answer, which is how a probe checks a question. Raw result pages go to target/conformance/raw/.
# The pace is fixed at one request per second (docs/internal/testing/conformance.md, D11).
#
# Usage: cargo run -q --example conformance_points -- <year_option|all> <kinks|samples> | scripts/conformance/fetch.sh [--dry-run]
set -euo pipefail

dry_run=0
case "${1:-}" in
  "") ;;
  --dry-run) dry_run=1 ;;
  *)
    echo "usage: $0 [--dry-run] < cases.csv" >&2
    exit 2
    ;;
esac

root="$(cd "$(dirname "$0")/../.." && pwd)"
fixtures="$root/tests/fixtures/conformance"
runs="$fixtures/runs.csv"
form_url="https://www.bmf-steuerrechner.de/ekst/eingabeformekst.xhtml"
pause=1 # D11: at most one request per second. A consent condition, so it is not configurable.
refresh_every=50
attempts=3
header="case_id,year_option,filing_status,taxable_income_cents,income_tax_cents,role,run_id"

work="$(mktemp -d "${TMPDIR:-/tmp}/conformance-fetch.XXXXXX")"
trap 'rm -rf "$work"' EXIT
jar="$work/cookies.txt"
cases="$work/cases.csv"

cat > "$cases"
if [ ! -s "$cases" ]; then
  echo "no cases on stdin" >&2
  exit 0
fi

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

# Loads what one fixture already has: the (filing,cents) keys, and the number of rows. Creates the fixture
# with its header when it is new. Stops when the case ids are not sequential, because the next id depends on that.
prepare_option() {
  local option="$1" fixture="$fixtures/$1.csv" rows last
  if [ ! -f "$fixture" ]; then
    echo "$header" > "$fixture"
  fi
  awk -F, 'NR > 1 { print $3 "," $4 }' "$fixture" > "$work/keys-$option.txt"
  rows="$(wc -l < "$work/keys-$option.txt" | tr -d ' ')"
  if [ "$rows" -gt 0 ]; then
    last="$(tail -n 1 "$fixture" | cut -d, -f1)"
    if [ "$last" != "$(printf '%s-%04d' "$option" "$rows")" ]; then
      echo "case ids in $fixture are not sequential; stop and check the file" >&2
      exit 1
    fi
  fi
  echo "$rows" > "$work/rows-$option.txt"
}

if [ ! -f "$runs" ]; then
  echo "run_id,papversion,fetched_at,source" > "$runs"
fi

run_id="run-$(date -u +%Y%m%dT%H%M%SZ)"
if [ "$dry_run" -eq 1 ]; then
  raw_dir="$root/target/conformance/raw/dry-$run_id"
else
  raw_dir="$root/target/conformance/raw/$run_id"
fi
mkdir -p "$raw_dir"

new_session
papversion="$(htmlq -a src 'script[src^="/papversion/"]' -f "$work/form.html" | sed -E 's#.*papversion_([0-9.]+)\.js.*#\1#')"

logged=0
fetched=0
skipped=0
requests=0
while IFS=, read -r option filing cents role; do
  if [ -z "$option" ]; then
    continue
  fi
  if [ "$dry_run" -eq 0 ]; then
    if [ ! -f "$work/rows-$option.txt" ]; then
      prepare_option "$option"
    fi
    if grep -qxF "$filing,$cents" "$work/keys-$option.txt"; then
      skipped=$((skipped + 1))
      continue
    fi
    if [ "$logged" -eq 0 ]; then
      printf '%s,%s,%s,%s\n' "$run_id" "$papversion" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" bmf_ekst_form >> "$runs"
      echo "$run_id: fetching from the BMF calculator (papversion $papversion)" >&2
      logged=1
    fi
  fi

  requests=$((requests + 1))
  if [ $((requests % refresh_every)) -eq 0 ]; then
    new_session || true
  fi
  raw="$raw_dir/$option-$filing-$cents.html"
  tax=""
  ok=0
  for attempt in $(seq 1 "$attempts"); do
    if tax="$(query "$option" "$filing" "$cents" "$raw")"; then
      ok=1
      break
    fi
    echo "retry $option $filing $cents (attempt $attempt of $attempts)" >&2
    sleep $((attempt * 5))
    new_session || true
  done
  if [ "$ok" -ne 1 ]; then
    echo "failed $option $filing $cents; rerun the same command to resume" >&2
    exit 1
  fi

  if [ "$dry_run" -eq 1 ]; then
    printf '%s,%s,%s,%s,%s\n' "$option" "$filing" "$cents" "$role" "$tax"
  else
    rows="$(cat "$work/rows-$option.txt")"
    rows=$((rows + 1))
    printf '%s-%04d,%s,%s,%s,%s,%s,%s\n' "$option" "$rows" "$option" "$filing" "$cents" "$tax" "$role" "$run_id" >> "$fixtures/$option.csv"
    echo "$filing,$cents" >> "$work/keys-$option.txt"
    echo "$rows" > "$work/rows-$option.txt"
    fetched=$((fetched + 1))
    echo "ok $option $filing $cents $tax" >&2
  fi
  sleep "$pause"
done < "$cases"

if [ "$dry_run" -eq 1 ]; then
  echo "dry run: $requests requests, nothing written" >&2
else
  echo "$run_id: fetched $fetched, skipped $skipped already in the fixtures" >&2
fi
