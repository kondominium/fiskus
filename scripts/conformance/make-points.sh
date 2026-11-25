#!/usr/bin/env bash
# Disclaimer: independent reference engine, not official BMF or ELSTER software, not tax advice.
# Haftungsausschluss: unabhängige Referenzimplementierung, keine offizielle Software des BMF oder von ELSTER, keine Steuerberatung.
#
# Writes a points list to stdout: case_id,year_option,filing_status,taxable_income_cents,role
# The zone starts are the first whole-euro single taxable incomes of zones 1 to 4 (see docs/internal/testing/conformance.md).
# Joint starts are twice the single starts. Interior points are seeded, so the list is reproducible.
#
# Usage: make-points.sh <year_option> <zone1_eur> <zone2_eur> <zone3_eur> <zone4_eur> [seed]
set -euo pipefail

if [ "$#" -lt 5 ] || [ "$#" -gt 6 ]; then
  echo "usage: $0 <year_option> <zone1_eur> <zone2_eur> <zone3_eur> <zone4_eur> [seed]" >&2
  exit 2
fi

awk -v year="$1" -v b1="$2" -v b2="$3" -v b3="$4" -v b4="$5" -v seed="${6:-1}" '
function emit(filing, cents, role) {
  n++
  printf "%s-%04d,%s,%s,%d,%s\n", year, n, year, filing, cents, role
}
BEGIN {
  print "case_id,year_option,filing_status,taxable_income_cents,role"
  srand(seed)
  bound[1] = b1; bound[2] = b2; bound[3] = b3; bound[4] = b4
  for (f = 0; f < 2; f++) {
    filing = (f == 0) ? "single" : "joint"
    mult = (f == 0) ? 1 : 2
    for (z = 1; z <= 4; z++) start[z] = bound[z] * mult
    emit(filing, 0, "zero")
    for (z = 1; z <= 4; z++) {
      emit(filing, (start[z] - 1) * 100, "kink_minus")
      emit(filing, start[z] * 100, "kink")
      emit(filing, start[z] * 100 + 99, "kink_cents")
      emit(filing, (start[z] + 1) * 100, "kink_plus")
    }
    for (z = 1; z <= 4; z++) {
      lo = start[z]
      hi = (z < 4) ? start[z + 1] - 1 : 10000000
      for (i = 0; i < 6; i++) {
        euros = lo + int(rand() * (hi - lo + 1))
        emit(filing, euros * 100 + int(rand() * 100), "interior")
      }
    }
    emit(filing, 100000000, "tail")
    emit(filing, 1000000000, "tail")
  }
}
'
