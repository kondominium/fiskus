---
name: add-tax-year
description: Add a new assessment year to fiskus (basic allowance zone). Covers sourcing, files, docs, review and tests.
---

# Add a tax year

Use this when the task is "add VZ <year> to fiskus". The engine implements only the
Grundfreibetrag zone of § 32a EStG, so a new year means a new basic allowance value
plus the matching wiring. Follow these steps in order.

## 0. Input

The target year is given as a four-digit calendar year, e.g. `2027`. Call it `Y`.

## 1. Verify the source (trustworthy, in this order)

1. **Primary (statute):** the Bundesgesetzblatt text of the law that sets the amount.
   - For recent years (2023+): recht.bund.de, e.g. `https://www.recht.bund.de/bgbl/1/<year>/<nr>/VO`.
   - For older years: dejure.org lists every law amending § 32a EStG with its BGBl page (index only, not a statute): https://dejure.org/gesetze/EStG/32a.html.
   - The amount appears as "bis <N> Euro (Grundfreibetrag)" in § 32a Abs. 1 Satz 2 Nr. 1 EStG.
2. **Official (BMF):** the Programmablaufplan (PAP) PDF states the Grundfreibetrag as `GFB = <N>`. BMF URLs share the base `https://www.bundesfinanzministerium.de/Content/DE/Downloads/Steuern/Steuerarten/Lohnsteuer/Programmablaufplan/`.
3. **Official report:** the Bundestag Existenzminimumbericht confirms the value for some years.
4. Cross-check the candidate value against `docs/references/README.md` and the existing `rules/*.toml` to spot a typo.

Rules:
- Never trust a number from memory or an LLM. Quote the statute or PAP.
- Cite one section per citation, in the form `§ 32a Abs. 1 Satz 2 Nr. 1 EStG`.
- Euro amounts in docs are written `12 348 €` (space between thousands).

## 2. Build / copy / prepare

1. Download the source PDF with `curl -sSL`, check it with `file`, record its sha256 with `shasum -a 256`.
2. Name it with the normalized scheme: `<year>-<source>-<name>-<ref>.pdf` (drop the `secondary` marker). Examples: `2024-bgbl-existenzminimum-I-386.pdf`, `2006-bmf-pap-2006.pdf`.
3. Place it in `docs/references/`.
4. Add a row to the table in `docs/references/README.md` (file, source, used for, status, sha256, and retrieved-from URL). Update the "Adding a source" note if the scheme changed.

## 3. Files to edit

1. `rules/<Y>.toml` — new file:
   ```
   # Tax parameters for VZ <Y>. Every value needs a citation and source.

   [basic_allowance]
   value_eur = <N>
   citation = "§ 32a Abs. 1 Satz 2 Nr. 1 EStG (...)"
   source = "<url>"
   retrieved = "<date>"
   reference = "https://gitlab.com/kondominium/fiskus/-/raw/main/docs/references/<file>.pdf"
   ```
2. `build.rs` — add `Y` to `YEARS`.
3. `src/year.rs` — add `Y<Y>` variant, a `ALL` entry, a `year()` arm, and a `from_year()` arm.
4. `src/params.rs` — add a match arm in `for_year`.
5. `tests/public_api.rs` — extend the `claimed` range `(2006..=Y)` if `Y` is the new top year; the value table lives in the unit test.
6. `src/tariff.rs` unit tests — add `(TaxYear::Y<Y>, <N>)` to `basic_allowance_per_year_matches_the_sources`.
7. `examples/basic_allowance.rs` — only if the example should demonstrate the new year; usually leave VZ 2026.
8. `docs/*` — see section 4.

The build fails if an entry has an unknown field, a negative or non-whole `value_eur`, a blank field, or a non-snake_case key. `make lint` fails if a `Parameters` field is unread.

## 4. Docs and how to think about changing

Think: "fiskus implements the basic allowance for VZ 2006 to <Y>". Update only what claims a year or a layout:

- `README.md` and `docs/README.md`: the implemented range.
- `docs/roadmap.md`: "except the basic allowance ... for tax years 2006 to <Y>".
- `docs/guide/00-intro.md`: "tax years 2006 to <Y>".
- `docs/guide/02-tax-year.md`: the supported range and `TaxYear::Y...` variants.
- `docs/usage/README.md`: the supported range and `TaxYear::Y2006` to `TaxYear::Y<Y>`.
- `docs/formulas/grundfreibetrag.md`: keep the VZ 2026 worked example; note that other years use their own `G` from `rules/<year>.toml`.
- `docs/internal/build/README.md`: only if the file layout changed (it did not).
- `docs/contributing/code.md`: only if the rules or steps changed.

Rules from `docs/contributing/language.md`:
- English prose, German statute quotes stay German.
- English term first, German in parentheses, on first use per page.
- `fiskus` in lowercase, including at sentence start.
- No "we", "our", "us". Make `fiskus` the subject.
- Cite one section per citation.
- Euro amounts as `12 348 €`.
- Say what fiskus implements; mark the rest "not yet in fiskus".

## 5. Review e2e

- Read the PDF and confirm the value, citation and source match.
- Confirm the `reference` URL resolves to the archived file on GitLab's main branch.
- Confirm `docs/references/README.md` row matches the file (sha256, status).
- Confirm no stray `rules/<Y>/parameters.toml` directory layout remains.
- Run `cargo package --list` and confirm `rules/<Y>.toml` is included.
- Grep for stale year strings (`2026 only`, `one variant`, `parameters.toml`, `rules/<year>/`) across docs.

## 6. Test e2e

Run `make gates` (lint, test-all, test-coverage, docs). All must pass. Specifically:

- `make test-unit`: `basic_allowance_per_year_matches_the_sources` and `every_year_is_tax_free_up_to_its_basic_allowance` pass.
- `make test-api`: `claimed_years_exist` and `out_of_range_years_are_not_supported` pass.
- `make test-guide`: `parameter_references_exist` passes (URL + local file).
- `make test-doc`: doc examples compile.
- Coverage stays at or above 90%.
- `make bench` compiles (it iterates `TaxYear::ALL`).

## 7. Limits

- Years before the euro (pre-2002, DM amounts) are not compatible today: the engine uses integer cents and assumes euro. Do not add them.
- Years before 2005 have no archived PAP in `docs/references`; VZ 2005 was sourced from the Haushaltsbegleitgesetz 2004 draft. Sourcing earlier years needs a new search (see the "Not yet in fiskus" section of `docs/references/README.md`).
- Only the basic allowance zone is implemented. Adding the full tariff (zones 2-5 of § 32a) is a separate slice.
- `TaxYear::from_year` returns `None` for unsupported years; `assess` cannot be called with them.
- Never commit, push or merge. Hand over the staged slice, a commit message draft (use the `commit-message` skill) and the gate result.
---

## Agents

Load this skill when the user asks to add a tax year. Do not restate its rules; point at `docs/contributing/code.md` and `docs/contributing/workflow.md` for gates and commits.