#!/usr/bin/env bash
# Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
# Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
#
# Brings every year option in scripts/conformance/year-options.csv to TARGET cases, newest year first.
# Only options whose first year is FROM_YEAR or later are fetched: the euro years, from 2002 by default.
# A year without a points list gets sample points (make-sample-points.sh). A points list that is short
# is topped up with samples. Then fetch.sh fetches the missing cases. A failing year is reported
# and the run continues. The summary at the end lists every year that has not reached TARGET.
# Re-running resumes where it stopped. Each top-up has its own seed, so it draws new incomes.
#
# Usage: fetch-all.sh [target_per_year] [from_year]   (defaults: 500, 2002)
set -euo pipefail

target="${1:-500}"
from_year="${2:-2002}"
root="$(cd "$(dirname "$0")/../.." && pwd)"
years="$root/scripts/conformance/year-options.csv"
fixtures="$root/tests/fixtures/conformance"
scripts="$root/scripts/conformance"
failed=()
short=()

while IFS=, read -r year_option _label first_year _last_year; do
  if [ "$year_option" = year_option ] || [ "$first_year" -lt "$from_year" ]; then
    continue
  fi
  points="$scripts/points/$year_option.csv"
  mkdir -p "$scripts/points"
  if [ ! -f "$points" ]; then
    "$scripts/make-sample-points.sh" "$year_option" $((target / 2)) $((year_option * 10000)) > "$points"
  fi
  have=$(($(wc -l < "$points") - 1))
  if [ "$have" -lt "$target" ]; then
    need=$((target - have))
    "$scripts/make-sample-points.sh" "$year_option" $(((need + 1) / 2)) $((year_option * 10000 + have)) $((have + 1)) | tail -n +2 >> "$points"
  fi
  echo "== year $year_option: $(($(wc -l < "$points") - 1)) cases in $points"
  if ! "$scripts/fetch.sh" "$points" < /dev/null; then
    echo "== year $year_option failed; re-run to resume" >&2
    failed+=("$year_option")
  fi
done < "$years"

echo "== summary: years from $from_year, target $target"
while IFS=, read -r year_option _label first_year _last_year; do
  if [ "$year_option" = year_option ] || [ "$first_year" -lt "$from_year" ]; then
    continue
  fi
  fixture="$fixtures/$year_option.csv"
  have=0
  if [ -f "$fixture" ]; then
    have=$(($(wc -l < "$fixture") - 1))
  fi
  echo "$year_option $have"
  if [ "$have" -lt "$target" ]; then
    short+=("$year_option:$have")
  fi
done < "$years"

if [ "${#short[@]}" -gt 0 ]; then
  echo "short of $target: ${short[*]}" >&2
fi
if [ "${#failed[@]}" -gt 0 ]; then
  echo "failed years: ${failed[*]}" >&2
fi
if [ "${#failed[@]}" -gt 0 ] || [ "${#short[@]}" -gt 0 ]; then
  exit 1
fi
echo "all years from $from_year reached $target"
