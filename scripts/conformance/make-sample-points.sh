#!/usr/bin/env bash
# Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
# Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
#
# Writes a sample points list to stdout: case_id,year_option,filing_status,taxable_income_cents,role
# Use it for years whose zone starts are not known yet. The samples are spread over four income bands,
# so every zone of the tariff gets cases even when its boundaries are unknown. The list is seeded,
# so it is reproducible. Use make-points.sh instead when the zone starts are known.
#
# Usage: make-sample-points.sh <year_option> <per_filing> [seed] [first_seq]
#   per_filing  cases per filing status, including the zero case when first_seq is 1
#   first_seq   sequence number of the first case, default 1 (a zero case is written only then)
set -euo pipefail

if [ "$#" -lt 2 ] || [ "$#" -gt 4 ]; then
  echo "usage: $0 <year_option> <per_filing> [seed] [first_seq]" >&2
  exit 2
fi

awk -v year="$1" -v per="$2" -v seed="${3:-1}" -v first="${4:-1}" '
function emit(filing, cents, role) {
  printf "%s-%04d,%s,%s,%d,%s\n", year, seq, year, filing, cents, role
  seq++
}
BEGIN {
  print "case_id,year_option,filing_status,taxable_income_cents,role"
  srand(seed)
  seq = first
  lo[1] = 1; hi[1] = 15000
  lo[2] = 15001; hi[2] = 70000
  lo[3] = 70001; hi[3] = 300000
  lo[4] = 300001; hi[4] = 10000000
  for (f = 1; f <= 2; f++) {
    filing = (f == 1) ? "single" : "joint"
    samples = per
    if (first == 1) {
      emit(filing, 0, "zero")
      samples = per - 1
    }
    for (i = 0; i < samples; i++) {
      band = (i % 4) + 1
      euros = lo[band] + int(rand() * (hi[band] - lo[band] + 1))
      emit(filing, euros * 100 + int(rand() * 100), "sample")
    }
  }
}
'
