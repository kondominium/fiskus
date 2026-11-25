# Conformance

Internal. Not part of the published guide. fiskus is not official BMF or ELSTER software, and nothing here is tax advice.

Status: living document. Last reviewed 2026-11-24. Each decision below has a status. Change a decision by editing its entry and its date, not by deleting it, so the reasons stay visible.

This page explains the conformance layer: what it checks, how its data is built and run, and why it is built that way. The rules for writing tests are in [code gates and rules](../../contributing/tests.md).

## What it checks

Conformance compares fiskus with results from the income tax calculator (Einkommensteuerrechner) of the Federal Ministry of Finance (BMF). For each case it gives fiskus a taxable income and a filing status and checks the income tax it computes against the calculator's answer, to the cent.

The calculator is independent of fiskus's reading of the statute, so a case catches a misreading of § 32a EStG that fiskus's own tests would repeat. Only the income tax row (Einkommensteuer) of the result is used. The total (Summe) adds the solidarity surcharge (Solidaritätszuschlag), which is out of scope.

The BMF has confirmed that fiskus may use its calculator output as test data. The request rate stays at one request per second or less.

## Current state

| Item | Value |
|---|---|
| Tax years | 20 euro years, 2002 to 2026 (the dropdown options whose first year is 2002 or later) |
| Cases per year | 500 (target reached). Minimum 300 (enforced by the test) |
| Cases in total | 10,000 |
| Tariff zones in fiskus | Only the basic allowance zone (zone 0) for 2026 |
| `make test-conformance` | Red by design. Every case that fiskus cannot compute is a failure |

The scoreboard that `make test-conformance` prints shows, per year: the cases, the passed cases, the failed cases, the zone-missing cases (tariff zone not yet in fiskus), the mismatched cases (fiskus disagrees with the calculator), and the passed share, with a total row.

## Decisions

### D1. Where conformance sits among the layers

Status: accepted.

| Layer | Where | Gate | Guarantees today |
|---|---|---|---|
| Conformance | `tests/conformance/main.rs`, fixtures in `tests/fixtures/conformance/` | `make test-conformance`, behind the `conformance` feature (see D3) | Each case matches the calculator to the cent. Red today: fiskus has only the basic allowance zone |

Conformance covers the data, and the other layers cover the formulas. Kani proofs cover a formula across its bounds, and conformance covers real incomes against the official answer. Neither replaces the other. The other layers are listed in [code gates and rules](../../contributing/tests.md).

### D2. The calculator is an oracle for behaviour, not a source of parameters

Status: accepted.

The parameters come from the statute, with a citation in `rules/`. The conformance tests show whether fiskus agrees with the calculator. They never supply a value that is then copied into `rules/`. Expected values in the fixtures come from the calculator, and no test copies a value from fiskus's own output. See the bad patterns in [code gates and rules](../../contributing/tests.md).

### D3. Conformance is opt-in behind the `conformance` feature

Status: accepted. Revisit when every tariff zone exists.

Context: `make gates` must pass before every handover, and conformance cannot pass until fiskus implements every zone of § 32a EStG.

Decision: the conformance test is a separate target, `tests/conformance/main.rs`, with `required-features = ["conformance"]` in `Cargo.toml`. `cargo test --all-targets` skips it, so `make gates` stays green. `make test-conformance` enables the feature and runs it.

Consequences:
- `make test-conformance` fails until fiskus has every tariff zone. Its scoreboard is the to-do list for the tariff zones.
- When every case passes, decide whether to drop the feature gate, so conformance joins `make gates`. Until then, it stays opt-in.

### D4. Conformance is strict

Status: accepted.

Decision: every case must match fiskus to the cent. A case that fiskus cannot compute yet is a failure, labelled "tariff zone not in fiskus yet", not a skipped case. The scoreboard counts the passed cases, the zone-missing cases, and the mismatched cases separately.

Why: a skipped case disappears from view. A failing case is a visible gap, and the scoreboard shows which years have gaps and how many.

### D5. Fixtures are committed, normalized, and in integer cents

Status: accepted.

- Money is integer cents, as in `Cents`. The fixtures never use floats or German number formats.
- Each row is one case, with its inputs (`year_option`, `filing_status`, `taxable_income_cents`) and its expected output (`income_tax_cents`). Values that repeat across rows live in `runs.csv` (the fetch that produced each row) and in `scripts/conformance/year-options.csv` (what each dropdown value covers).
- Older years can return cents, so `income_tax_cents` is not always a multiple of 100. Compare the exact cents.
- The fixtures are committed, so the test runs offline.

### D6. Case selection: the rules give the kinks, seeded samples give the rest

Status: accepted. Revised 2026-11-26 (slice 2.1).

- **Kinks.** `scripts/conformance/points.rs`, through `make conformance-cases KIND=kinks`, takes the basic allowance and each zone start from the rules. For each one it writes the income one euro below, at, with 0.99 € more, and one euro above, for single and for joint filing. It adds zero, one odd joint income (the halving rounds it down), and two tail incomes. A wrong start in `rules/` shows as a mismatch, and the calculator stays the oracle (D2).
- **Samples.** `KIND=samples` writes `COUNT` seeded incomes per filing status, spread over four income bands, so every zone gets cases between its boundaries.
- **Seeds.** The default seed is the year option times 10 000, plus the cases its fixture already has. A top-up therefore draws incomes the fixture does not have yet. A seed can be given as the fourth argument, and the same seed gives the same list.
- **The 2002 option** covers 2002 and 2003. Its kink points use the 2003 rules, including the basic allowance that the plan checks in slice 3.2. Do not treat those kinks as verified until that slice is done.

### D7. Tests read fixtures at compile time

Status: accepted.

`tests/` must not read files at runtime, because a test that reads files can hide a dependency. The conformance test embeds each fixture with `include_str!`. A new year needs a new entry in `FIXTURES`, so a missing year cannot silently drop out of the test. A missing file fails the build.

### D8. The result is the income tax row, not the total

Status: accepted.

The BMF result page has a row for the income tax (Einkommensteuer) and a total (Summe). The total adds the solidarity surcharge (Solidaritätszuschlag), which is out of scope. The fetch reads the income tax row only. Found during the first 2026 fetch, which read the total and recorded 111 355.25 € where the income tax was 105 550 €. The bad rows were discarded and refetched.

### D9. Income tax is not always whole euros

Status: accepted. Observed.

The 2026 results come in whole euros. The probe of older years returned cents: for example, 2001 returned 14 445.52 € for 50 000 € single. So `income_tax_cents` is not always a multiple of 100, and no test may assume it is. Whether each year rounds in the same way is an open question, see Open questions.

### D10. Raw pages stay out of the repo

Status: accepted. Revised 2026-11-26 (slice 2.1).

`scripts/conformance/fetch.sh` writes each raw result page under `target/conformance/raw/<run_id>/`, which git ignores, so a result can be audited without committing the page. `--dry-run` prints each answer and writes nothing. A probe uses it to check a question without changing a fixture.

### D11. Consent and rate

Status: accepted. Consent confirmed by the BMF. Date: `<YYYY-MM-DD>` (to fill in).

The fetch sends at most one request per second and never rotates IP addresses or works around protection. If the BMF changes its terms, stop and check before fetching again. Questions go to Steuerrechner@bmf.bund.de, listed in the BMF legal notice (Impressum).

### D12. Names are source-neutral

Status: accepted. Done on the `tests/conformance` branch.

Decision: names of tests, targets, features, tasks and modules never name a data source. The test, feature and tasks say "conformance", so a second source (another calculator, or the published program) joins the same target. The source is named only where the data comes from: the form field names and the source URL in `fetch.sh`, and the `source` column of `runs.csv`.

Why: a name that names one source would need renaming, and every reference updated, the day a second source arrives.

### D13. Data covers the euro years, 2002 to 2026

Status: accepted. Pre-euro years are parked, not dropped.

Decision: the fixtures cover the 20 dropdown options whose first year is 2002 or later (2026 down to 2002). The earlier options (2001 and before, including the 1958 to 1975 groups) are not fetched for now, and no test includes them. `make conformance-cases YEAR=all` covers the euro options, from 2002.

Why: the euro is the currency of every year in the scope, so the figures compare directly. The probe showed that the calculator returns results for every option, so the pre-euro years are a scope choice, not a technical gap. They can be fetched later, once their year options are in `year-options.csv`, and the conformance test then needs their entries in `FIXTURES`.

### D14. One test, with the scoreboard printed whole

Status: accepted. Revised 2026-11-26 (slice 2.1), replacing one test per year.

Decision: `tests/conformance/main.rs` has one test. It checks every selected year, prints the scoreboard in year order, and then fails with the messages for every year that has failing cases. `CONFORMANCE_YEARS` still picks the years. `make test-conformance` passes `--nocapture`, so the table shows.

Why: with one test per year, libtest ran the years concurrently, and their summary lines interleaved. A row could go missing from the scoreboard. In one gate run the 2023 row was lost, which had 500 passing cases, so the total read 9 500 cases instead of 10 000. One test prints the table from one place, in order.

### D15. At least 300 cases per year, target 500

Status: accepted. Set 2026-11-24.

Decision: every year needs at least 300 cases. The test fails a year with fewer, so a thin year cannot pass quietly. The fetch targets 500 per year, which gives the zones more cases to be tested against, and leaves room for the kink cases of the years whose zone starts become known.

Why: a few hundred cases per year still leaves most incomes unchecked, but it makes each year a real sample rather than a token. The count is set in the test, not only in the fetch, so a short fixture fails the test instead of being noticed later.

Cost: a full fetch at the BMF's rate limit takes several hours. The fetch resumes where it stopped and reports the years still short of target.

### D16. The scoreboard reports how close fiskus is

Status: accepted. Set 2026-11-24.

Decision: `make test-conformance` prints a table after the run: per year, the cases, the passed cases, the failed cases, the zone-missing cases, the mismatched cases, and the passed share, with a total row. The test process still exits red until every case passes (D4). The scoreboard is the progress measure, and it is the figure to report between runs.

Why: a red run with only a failure list hides the progress. The scoreboard shows whether a new zone moved the numbers, and by how much.

## Adding a tariff rule

Each new tariff rule ships with all of the following in the same slice:

1. A formula page in `docs/formulas/`, which states the rule and its boundaries.
2. Unit tests in `#[cfg(test)] mod tests`, each with a one-line `//` comment naming the case and its citation.
3. A `properties` module in the same file, with proptest coverage for the properties of the rule.
4. A Kani harness in `#[cfg(kani)] mod verification` when the function is pure: parameters passed in, not loaded from TOML.
5. A guide update in `docs/guide/` when the rule, parameter, zone or error changes.

Then run `make test-conformance`. The zone's row in the scoreboard should move. If it does not, read the mismatched cases before anything else: a mismatch is a finding about the formula or the reading of the statute, not a reason to edit a fixture.

## Data

Fixtures live in `tests/fixtures/conformance/`, one file per year option: `<year_option>.csv`. The cases to fetch are not stored. `points.rs` writes them when they are needed.

| File | Contents |
|---|---|
| `<year_option>.csv` | Cases: `case_id,year_option,filing_status,taxable_income_cents,income_tax_cents,role,run_id`. `case_id` is `<year_option>-NNNN`, numbered in order, so the next id is the row count plus one |
| `runs.csv` | One row per fetch that wrote cases: `run_id,papversion,fetched_at,source` |
| `scripts/conformance/year-options.csv` | One row per dropdown value, with its label and the years it covers |
| `scripts/conformance/points.rs` | The generator. It writes `year_option,filing_status,taxable_income_cents,role` lines to stdout |
| `scripts/conformance/fetch.sh` | The fetcher. It reads those lines on stdin, and appends the results to the fixtures |

Roles: `zero`, `kink_minus`, `kink`, `kink_cents` (start + 0.99 €), `kink_plus`, `odd` (an odd joint income), `tail`, and `sample`. The 2026 fixture was written before the `odd` role existed. Its `interior` rows stay as they are.

Current state (update after each full fetch):

| Year option | Cases | Notes |
|---|---|---|
| 2026 | 500 | All pass |
| 2025 to 2005 | 500 each | All pass |
| 2004 | 500 | All pass (parameters corrected from the BMF formula) |
| 2003 | 500 | Not yet in fiskus (zone_missing). Basic allowance only |
| 2002 | 500 | Not yet in fiskus (zone_missing). Basic allowance only |
| 2001 and earlier (18 options) | not fetched | Pre-euro years, parked (D13) |

A grouped option such as `2010` covers 2010 to 2012. The BMF treats the years in one group as one tariff. The fixture is per option, not per year.

## Operations

Commands are in `mise.toml` and the Makefile.

- Probe a question without writing anything: `make conformance-cases YEAR=2026 KIND=samples COUNT=2 DRY_RUN=1`. That is four requests, one per second. For one hand-written case, pipe a line into `scripts/conformance/fetch.sh --dry-run`.
- Fetch the kinks or the samples of an option: `make conformance-cases YEAR=2010 KIND=kinks`, or `KIND=samples COUNT=50`. A run resumes where it stopped. It reports how many cases it fetched and how many it skipped, because the fixture already had them.
- Run the conformance tests: `make test-conformance` runs years 2002 to 2025 and prints the scoreboard. Pick years with `make test-conformance YEARS=2010,2026`, or run everything with `YEARS=all`.
- After the BMF publishes a new `papversion`, probe a sample of kink and interior cases with `DRY_RUN=1`, and compare them with the fixtures. Fetch more only after that.

Adding a tax year:

1. Add the dropdown value to `scripts/conformance/year-options.csv`.
2. Check the zone starts against the statute, and confirm them with kink cases.
3. Fetch the kinks and the samples with `make conformance-cases`, and add the option's fixture to `FIXTURES` in `tests/conformance/main.rs`.
4. In fiskus, add the year to `TaxYear` and `rules/<year>.toml`, with a citation for each value. Write the formula page in `docs/formulas/` first.



## Open questions

- **Zone starts for older years.** Taken from the statute history, then checked with kink cases. Until then, those years have only samples, which cannot pin a boundary.
- **Rounding by year.** The 2001 result has cents, but 2026 has none. Check each year's rounding rules against the statute and the calculator's own explanation table on its result page.
- **Floating-point in the BMF calculator.** Two cases (2019-0215 and 2020-0335) differ by one to two euros. fiskus uses exact integer arithmetic (Fraction), while the BMF calculator uses floating point. The statute (§ 32a Abs. 1 Satz 6) requires rounding down the final result, which fiskus does. The BMF's floating-point computation gives a slightly lower result in these two cases. This is a known discrepancy, not a fiskus bug.
- **Plausibility of identical results.** Several 1990s options returned the same tax for 50 000 € single. This is probably correct, because the tariff did not change, but it needs a check against the tariff history before fiskus relies on it.
- **Cross-check with the published program.** Compare the calculator with the XML pseudocode for 2025 and 2026, to show that the form and the program agree.
- **Feature gate.** When every case passes, decide whether conformance joins `make gates` (D3).

## Status

| Item | Status |
|---|---|
| Conformance test (`tests/conformance/`) | Built, strict, gated by the `conformance` feature. One test for every selected year, with a 300-case minimum and a scoreboard |
| Generator and fetcher (`scripts/conformance/`) | `points.rs` writes the cases, and `fetch.sh` fetches them. Resumable, one request per second, with a dry run |
| Fixtures | Complete: 10,000 cases, 500 per euro year. See Data |
| BMF consent | Confirmed. Date to fill in (D11) |
| fiskus tariff zones | All zones for 2004-2026. 2003 (and the 2002 year-option) not yet implemented |
| Conformance as a gate | `make test-conformance` is part of `make gates`. It runs and reports, but does not block (2002 has 406 zone-missing, 2019 and 2020 have 1 floating-point discrepancy each) |

