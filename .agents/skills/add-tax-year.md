---
name: add-tax-year
description: Add a new assessment year to fiskus (basic allowance and tariff zones of § 32a EStG). Covers sourcing, files, docs, review and tests.
---

# Add a tax year

Use this when the task is "add VZ <year> to fiskus". A year needs its basic allowance (Grundfreibetrag). If its tariff zones are sourced, it also needs its tariff. Follow these steps in order.

## 0. Input

The target year is a four-digit calendar year, e.g. `2027`. Call it `Y`.

Before any edit, read the current range from code, not from this file: `YEARS` in `build.rs` and `TaxYear::ALL` in `src/year.rs`. The range the docs state must match them.

## 1. Verify the source (trustworthy, in this order)

1. **Primary (statute):** the Bundesgesetzblatt (BGBl.) text of the law that sets the amount and the zones.
   - 2023 and later: recht.bund.de, e.g. `https://www.recht.bund.de/bgbl/1/<year>/<nr>/VO`.
   - Earlier: dejure.org lists every law amending § 32a EStG with its BGBl page (an index only): https://dejure.org/gesetze/EStG/32a.html.
   - The basic allowance is written as "bis <N> Euro (Grundfreibetrag)" in § 32a Abs. 1 Satz 2 Nr. 1 EStG. The zones are the five formulas of § 32a Abs. 1 Satz 2 EStG.
2. **Official (BMF):** the Programmablaufplan (PAP) PDF states `GFB = <N>` and the zone formulas. BMF URLs share the base `https://www.bundesfinanzministerium.de/Content/DE/Downloads/Steuern/Steuerarten/Lohnsteuer/Programmablaufplan/`.
3. **Draft or report:** a Bundestag draft or report is a source of last resort. Mark it as a draft in the citation and in the docs, as VZ 2004 and 2005 are.
4. Cross-check the candidate values against `docs/references/README.md`, the neighbouring years in `rules/`, and the Lohnsteuer repository (a lead only, never a source).

Rules:
- Never trust a number from memory or from an LLM. Quote the statute or the PAP.
- Cite one section per citation, in the form `§ 32a Abs. 1 Satz 2 Nr. 1 EStG`.
- Euro amounts in docs are written `12 348 €`.

## 2. Build, copy and prepare

1. Download each source PDF with `curl -sSL`, check it with `file`, and record its sha256 with `shasum -a 256`.
2. Name it `<year>-<source>-<name>-<ref>.pdf`, without the `secondary` marker. Examples: `2024-bgbl-existenzminimum-I-386.pdf`, `2006-bmf-pap-2006.pdf`.
3. Place it in `docs/references/`.
4. Add a row to the table in `docs/references/README.md`: file, source, used for, status, and sha256.
5. Check every coefficient against the text of its PDF. Extract the text (Ghostscript: `gs -q -dNOPAUSE -dBATCH -sDEVICE=txtwrite -sOutputFile=<out.txt> <pdf>`) and search for each number in the form the PAP prints, e.g. `997,6` for 997,60. Note any value that does not appear. A hit only shows that the number is in the document. Confirm that it sits in the zone formula it belongs to, by reading the formula line.
6. Compute the expected values independently with exact fractions (Python `fractions.Fraction`), in a script under `.agents/research/` (git-ignored). Compare the script with the zones in `rules/<Y>.toml`. Do not take the expected values from fiskus's own output. Fixed figures do not go into the tests: they wait for the conformance layer.

## 3. Files to edit

1. `rules/<Y>.toml`, new file. Use this layout:

   ```toml
   # Tax parameters for VZ <Y>. Every value needs a citation and source.

   [basic_allowance]
   value_eur = <N>
   citation = "§ 32a Abs. 1 Satz 2 Nr. 1 EStG (...)"
   source = "<url>"
   retrieved = "<date>"
   reference = "https://github.com/kondominium/fiskus/raw/main/docs/references/<file>.pdf"

   [source]
   citation = "§ 32a Abs. 1 Satz 2 EStG (...)"
   source = "<url>"
   retrieved = "<date>"
   reference = "https://github.com/kondominium/fiskus/raw/main/docs/references/<file>.pdf"

   [[zone]]
   from_eur = <first euro of zone 2>
   base_eur = <allowance, or the start of the quadratic variable>
   step_eur = 10000
   quadratic_h = <a in hundredths>
   linear_h = <b in hundredths>
   constant_h = <c in hundredths>
   ```

   If the statute sets the solidarity threshold of the year (§ 3 Abs. 3 SolZG), add a `[solidarity]` table with `free_threshold_single_eur`, `free_threshold_joint_eur`, `citation`, `source`, `retrieved` and `reference`. Read the amounts from the archived statute text, and cite the article of the amending law. Without the statute archived, leave the table out: the surcharge is then `None` for the year.

   Repeat `[[zone]]` for each zone. Quadratic zones use `step_eur = 10000`, linear zones use `step_eur = 1` with `quadratic_h = 0`. A year without a sourced tariff has `[basic_allowance]` only, with no `[source]` and no `[[zone]]`.
2. `build.rs`: add `Y` to `YEARS`. Keep the list in order.
3. `src/year.rs`: add the `Y<Y>` variant with its doc comment, its entry in `ALL` (update the array length), a `year()` arm, and a `from_year()` arm.
4. `src/params.rs`: nothing. `for_year` and `tariff_for` are generated from `YEARS`.
5. `src/tariff.rs` and `tests/public_api.rs`: nothing. The tests read the year list and the rule data, and run for every listed year. Do not add fixed figures for the year here. They belong to the conformance layer.
6. `examples/`, `benches/` and `scripts/guide/charts.rs`: nothing. They use the latest listed year, so the new year is picked up.
7. `docs/formulas/`: nothing. The formula pages hold formulas and point to `rules/`, with no per-year table.
8. Docs, see step 4. The solidarity thresholds show in `docs/formulas/solidarity-surcharge.md` and in the references table.

The build fails on a missing `[source]` for a year with zones, a first zone that does not start at the allowance plus one euro, zones out of order, a step that is not positive, or a tax jump of more than one euro at a zone start.

## 4. Docs

Update the range only where a page claims it. Do not copy the range from this file. Read it from the code (step 0), then update:

- `README.md` and `docs/README.md`: the implemented range.
- `docs/roadmap.md`: the range in the intro, and the "Done" paragraph of milestone 4.
- `docs/guide/README.md`: the range and the `TaxYear` variants.
- `docs/guide/README.md`: only if the zero-tax figures change, which they do for a new basic allowance. Recompute them from the code and check the page against `make charts`.
- `docs/guide/glossary.md`: register every new German term before a page uses it, under its first letter.
- `docs/guide/coverage.toml`: if a guide page cites a new `§`, list it for that page.
- `docs/usage/README.md`: the range, and the example values.
- `docs/formulas/grundfreibetrag.md`: the sample data is not a year, so it does not change.
- `docs/references/README.md`: the sources table, and the tariff section if the new year has a tariff.
- `docs/internal/benchmarks.md`: re-measure with `make bench` if the bench inputs change. Do not keep figures from an older run.

Writing rules from `docs/contributing/language.md`:
- English prose, German statute quotes stay German.
- English term first, German in parentheses, on first use per page.
- `fiskus` in lowercase, including at sentence start.
- No "we", "our", "us". Make `fiskus` the subject.
- Say what fiskus implements. Mark the rest "not yet in fiskus".

## 5. Review end to end

- Read each source PDF and confirm the basic allowance, each zone's formula and the zone limits.
- Confirm each `reference` is the GitHub main-branch link form, and that its file name is in `docs/references/`, with its sha256 in the README. `parameter_references_exist` checks both.
- Build the crate's package list (`cargo package --list`). `rules/<Y>.toml` must appear. `docs/references/` is not packaged, and `build.rs` does not read it, so it can stay out of the package.
- Grep for stale strings across the docs: `one variant`, `parameters/`, `OutsideImplementedRange` where the text says it is the zone limit, and `basic allowance zone` where the tariff is meant.

## 6. Test end to end

Run `make gates` (lint, test-all, test-coverage, docs). All must pass. Check these in particular:

- `make lint`: rustfmt and clippy with warnings denied.
- `make test-unit`: the property tests over `TaxYear::ALL`, and `every_year_is_zero_up_to_its_basic_allowance` and `every_zone_joins_the_previous_one_within_one_euro`.
- `make test-api`: `listed_years_are_contiguous_and_round_trip` and `years_outside_the_listed_range_are_not_supported`.
- `make test-proofs`: the Kani harnesses pass. The tariff harness is bounded to 10^12 euros.
- `make test-all` runs the guide checks: `parameter_references_exist`, `parameter_values_appear_on_the_page_that_cites_them`, the chart drift check and the chapter checks. The guide checks must actually read the rules. If a check reports zero parameters, it is broken, not passing.
- `make test-doc`: the doc examples compile.
- Coverage stays at 90% or above.
- `make bench` compiles and runs when the bench inputs changed.

## 7. Limits

- Years before the first sourced year: no archived source. Sourcing an earlier year needs a new search. The "Not yet in fiskus" section of `docs/references/README.md` lists what is missing.
- Amounts before the euro (DM) are not compatible: the engine uses integer cents and assumes euros. Do not add them.
- A year without archived zones has no tariff. Its basic allowance alone is implemented, and income above that allowance returns `Error::OutsideImplementedRange`. Check the current state in `docs/roadmap.md`.
- A draft text stays marked as a draft until the enacted text is archived.
- `TaxYear::from_year` returns `None` for unsupported years, so `assess` cannot be called with them.
- Never commit, push or merge. Hand over the staged slice, a commit message draft (use the `commit-message` skill), and the gate result.

## Pointers

Gates and the rules for code: `docs/contributing/code.md`. Tests: `docs/contributing/tests.md`. Commits: `docs/contributing/workflow.md`.
