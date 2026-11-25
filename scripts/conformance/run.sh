#!/usr/bin/env bash
# Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
# Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
#
# Runs the conformance tests and prints a scoreboard: how many cases each year passes and fails, and why.
# It also reports how long the test process took and how much memory it peaked at. Build time is not included.
# The full output is kept in target/conformance/last-run.log, and each run is appended to timing.log.
#
# Usage: [CONFORMANCE_YEARS=2010,2026|all] [RUST_TEST_THREADS=n] run.sh
set -euo pipefail

root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$root"

binary="$(cargo test --test conformance --features conformance --no-run 2>&1 \
  | sed -n 's/^ *Executable .*(\(.*\))$/\1/p' | tail -n 1)"
if [ -z "$binary" ]; then
  echo "could not find the conformance test binary" >&2
  exit 2
fi

mkdir -p target/conformance
log="target/conformance/last-run.log"
scoreboard="target/conformance/scoreboard.txt"
timing="$(mktemp "${TMPDIR:-/tmp}/conformance-time.XXXXXX")"
trap 'rm -f "$timing"' EXIT

status=0
/usr/bin/time -l -o "$timing" "$binary" --nocapture "$@" 2>&1 | tee "$log" || status=$?

awk '
$1 == "score" {
  for (i = 2; i <= NF; i++) {
    split($i, kv, "=")
    v[kv[1]] = kv[2]
  }
  if (v["year"] != "") {
    print v["year"], v["cases"], v["passed"], v["zone_missing"], v["mismatched"]
  }
}' "$log" | sort > "$scoreboard"

if [ -s "$scoreboard" ]; then
  echo
  echo "conformance scoreboard (zone missing: the tariff zone is not in fiskus yet; mismatched: fiskus disagrees with the calculator)"
  awk '
  BEGIN {
    printf "%-6s %7s %8s %8s %13s %11s %8s\n", "year", "cases", "passed", "failed", "zone missing", "mismatched", "passed"
  }
  {
    cases += $2; passed += $3; zone += $4; mismatched += $5
    printf "%-6s %7d %8d %8d %13d %11d %7.1f%%\n", $1, $2, $3, $2 - $3, $4, $5, ($2 ? 100 * $3 / $2 : 0)
  }
  END {
    printf "%-6s %7d %8d %8d %13d %11d %7.1f%%\n", "total", cases, passed, cases - passed, zone, mismatched, (cases ? 100 * passed / cases : 0)
  }' "$scoreboard"
fi

real="$(awk '/ real / {print $1}' "$timing")"
rss_bytes="$(awk '/maximum resident set size/ {print $1}' "$timing")"
rss_mib="$(awk -v bytes="$rss_bytes" 'BEGIN {printf "%.1f", bytes / 1048576}')"
years="${CONFORMANCE_YEARS:-default, up to 2025}"
threads="${RUST_TEST_THREADS:-default}"
summary="$(date -u +%Y-%m-%dT%H:%M:%SZ) years=[$years] threads=$threads real=${real}s max_rss=${rss_mib}MiB exit=$status"

echo "$summary" >> target/conformance/timing.log
echo
echo "conformance run: $summary"
exit "$status"
