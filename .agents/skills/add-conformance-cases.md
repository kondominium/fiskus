---
name: add-conformance-cases
description: Add cases from the BMF income tax calculator to fiskus's conformance fixtures, with the scripts in scripts/conformance. Covers probes, kinks, samples, new year options, triage and hand-over.
---

# Add conformance cases

Use this when the task is to fetch results from the BMF income tax calculator into `tests/fixtures/conformance/`. The rules are in the Rules section below. This page also says how to run the scripts.

## When to use it

- **A probe:** settle one question about the calculator, such as whether a value is rounded. Use `DRY_RUN=1`. Nothing is written.
- **Kinks:** a new or changed zone or basic allowance. Fetch the points at each boundary, so a wrong value in `rules/` shows as a mismatch.
- **Samples:** widen a year's coverage with more incomes in the four bands.
- **A new year option:** the BMF has a new dropdown value. See "Adding a year option" below.

## Rules

- The calculator is the **behaviour oracle, not a source of parameters**. Never copy a figure from a result into `rules/`. A parameter comes from the statute, with its citation.
- Fixtures are **append-only**. Never edit or delete a row by hand. A mismatch is a finding about fiskus or the statute, not a reason to change the fixture.
- The pace is **one request per second**, and the scripts fix it. Do not change it, rotate IP addresses, or work around protection. If the BMF's terms change, stop and check them before fetching again. Questions go to Steuerrechner@bmf.bund.de, listed in the BMF legal notice.
- Raw result pages stay in `target/conformance/raw/`, which git ignores. Do not commit them.

## Setup

Run `mise install` once. It provides `htmlq` and the other tools the scripts use. The scripts need network access to `www.bmf-steuerrechner.de`.

## Steps

1. **Pick the option and the kind.** The option is a `year_option` from `scripts/conformance/year-options.csv`, such as `2026` or `2010` (which covers 2010 to 2012). The kind is `kinks` or `samples`. `YEAR=all` runs every euro option, from 2002.
2. **Probe first.** Fetch two samples, which is four requests. The form may have changed, and the probe shows it before a bulk fetch:

   ```bash
   make conformance-cases YEAR=2026 KIND=samples COUNT=2 DRY_RUN=1
   ```

   Each line of output is `option,filing,cents,role,tax`. Check that every line has a tax value. If the probe fails, go to "Troubleshooting".
3. **Fetch.** Run the same command without `DRY_RUN`. For kinks, use `KIND=kinks`, which takes no count. For samples, `COUNT=50` gives 50 single and 50 joint cases:

   ```bash
   make conformance-cases YEAR=2026 KIND=kinks
   make conformance-cases YEAR=2010 KIND=samples COUNT=50
   ```

   A run skips every case its fixture already has, so a rerun continues. If a run stops with `failed ...; rerun the same command to resume`, run the same command again. Do not change the case list.
4. **Check the diff.** The fetch only appends rows, and adds one line to `runs.csv`. Confirm it:

   ```bash
   git diff --stat tests/fixtures/conformance
   git diff tests/fixtures/conformance | grep '^-' | grep -v '^---'
   ```

   The second command must print nothing. Any removed line is a bug in the fetch. Stop and report it.
5. **Run the conformance test** for the option:

   ```bash
   make test-conformance YEARS=2026
   ```

   Read the scoreboard. Every case should pass, or be one of the open problems listed in the test. A mismatch is a finding.
6. **Triage a mismatch.** For each failing case, compare the fiskus value with the calculator's, and work out which side is wrong:
   - A zone start or coefficient in `rules/` differs from the statute: that is a rules bug. Fix it in its own slice, with the statute citation.
   - fiskus's formula or rounding differs from the statute: that is a code bug. Write a failing test first, then fix it in its own slice.
   - The difference is one euro, and the exact value is a whole euro: this may be the BMF's floating point. Do not list it yourself. Report it to the owner with both exact values, and the double-precision model that the conformance section of `docs/internal/testing/README.md` describes. The owner decides whether it goes into `OPEN_PROBLEMS` in `tests/conformance/main.rs`.
   - Never edit the fixture to make a case pass.
7. **Hand over.** Stage only the fixture files and `runs.csv`:

   ```bash
   git add tests/fixtures/conformance
   ```

   Then draft the commit message with the `commit-message` skill, and report the `make gates` result. Gates run conformance, and a case that differs from the calculator fails the gate unless it is one of the open problems, so say how many cases differ.

## Adding a year option

1. Add the dropdown value to `scripts/conformance/year-options.csv`, with its first and last year.
2. Add the year to fiskus with the `add-tax-year` skill. The option maps to one of fiskus's years, so the code needs that year first.
3. In `tests/conformance/main.rs`, add the option to `FIXTURES`, with an empty fixture file that has only the header. The tax years come from `year-options.csv`.
4. Fetch the kinks and the samples, as in steps 1 to 4.
5. Run `make test-conformance YEARS=<option>`, and triage as in step 6.

## Troubleshooting

| Symptom | Action |
|---|---|
| `form returned HTTP 503`, or other HTTP errors | Wait, then rerun the same command. The script retries three times on its own. |
| `no ViewState on the form` | The form has changed. Stop. Inspect the raw form under `target/conformance/raw/`, and report it. |
| `echoed zvE ... does not match` | The calculator read a different income. Stop, and report it. Do not fetch more. |
| `case ids in ... are not sequential; stop` | The fixture was edited by hand. Stop, and report it. Restore the file from git. |
| A new papversion appears in the run | The calculator has changed. Probe a sample of kinks and interior cases with `DRY_RUN=1`, compare them with the fixture, and only then fetch more. |

## Notes

- A probe for one hand-written case: pipe a line into the fetcher, for example `printf '2026,single,1234900,probe\n' | scripts/conformance/fetch.sh --dry-run`.
- The 2002 option covers 2002 and 2003, and its kink points use the 2003 rules, which are cited in `rules/2003.toml`. All 500 of its cases match the calculator.
