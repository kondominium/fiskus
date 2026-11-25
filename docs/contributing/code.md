# Code

Applies to humans and agents. For branches and commits, see [Workflow](workflow.md). For tests, see [Tests](tests.md).

## Gates

Run `make gates` before handover. It runs every gate CI runs, and CI runs the same targets on every push.

| Gate | Command | CI job |
|---|---|---|
| Lint: rustfmt and clippy, warnings denied | `make lint` | `lint` |
| Tests: unit, property, doc, public API, guide, proofs | `make test-all` | `test` |
| Coverage: lines at least 90% | `make test-coverage` | `coverage` |
| Docs: the book builds | `make docs` | `docs` |

- A change is not handed over with a failing gate. Hand over the result with the change.
- Scratch: research, drafts and plans go in `.agents/research/` and `.agents/plans/`. Both are git-ignored and never committed.
- Guide: a change that adds or alters a rule, parameter, zone or error updates `docs/guide/` in the same diff. See [guide](guide.md).

## Rules

- No `f64` for money: binary floats can't store decimals exactly (`0.1 + 0.2 != 0.3`), which breaks equality checks and rounding. Use integer cents or a decimal type.
- No `unwrap()` or `expect()` in library code reachable from user input. Return `Result`: bad input would panic the caller instead of returning an error it can handle.
- No runtime filesystem reads. Tax parameters are compiled in: the binary is self-contained and reproducible, and stray files can't silently change results.
- Provenance (the § citation and parameter version behind each number) is a planned separate feature. It is not in the code yet. The citations, sources and archived `reference` links stay in `rules/` as reference: readers shouldn't assume the code tracks sources it doesn't yet.
- Verify tax figures against the statute before relying on them. Tax law changes, and numbers from memory or an LLM are often wrong.
- Identifiers are English. Types, functions, fields, variables, constants, files, test names and parameter keys never use German, even where German is the precise tax term: one language keeps code consistent and searchable.
- Docs and doc comments follow the [language rules](language.md), including German terms in parentheses after the English term. Why: readers find the German term, while code keeps the English name.

## Lints

The lint rules live in `Cargo.toml` (`[lints]`) and `clippy.toml`. `make lint` denies every warning, for every target. `missing_docs`, the pedantic group and the safety lints (`unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `float_arithmetic`) all apply.

An exception is written as `#[expect(..., reason = "...")]` next to the code it covers, or as a file-level `#![expect(...)]` in a tool target. The reason names the fact that makes the exception safe, for example "the charts place points in SVG units". `expect` fails when its lint no longer fires, so an exception cannot outlive its cause. Generated code is the one place that uses `allow`, because its output cannot carry a comment.

Tests may unwrap, expect, panic, index and print (`clippy.toml`). Library code may not.

## Style and docs

- Prefer well-named functions over comment blocks. Name the step, don't narrate it. Comments are for the why the code can't say. Why: a function name is read at every call site, while a comment drifts from the code.

Bad:

```rust
// Joint filers are split in half, singles are taxed as-is.
// Round down to whole euros first, then compare against the allowance.
let basis_eur = match unit.filing {
    FilingStatus::Single => taxable_income_eur,
    FilingStatus::Joint => taxable_income_eur / 2,
};
```

Good:

```rust
let basis_eur = filing_basis_eur(&unit.filing, taxable_income_eur);
```

Bad: `let euros = cents / 100; // cents to euros`. Good: `let euros = cents.whole_euros();`.

- Every `pub` item has a `///` doc: what it does, its units (euros or cents), and the errors it returns. Why: external users read docs, not the source.

Good:

```rust
/// Assesses income tax for one filing unit in `year`.
///
/// Income is in cents. Returns [`Error::NegativeTaxableIncome`] for negative income
/// and [`Error::OutsideImplementedRange`] above the allowance when the tax cannot be computed.
pub fn assess(year: TaxYear, unit: &FilingUnit) -> Result<Assessment, Error>
```

Bad: `/// Assesses tax.`

- Doc examples go in a `/// # Examples` section, run by `make test-doc`. Use only the public API, not `crate::` paths. Why: doc examples show the public call shape, so they must compile against what users import.
- Pertinent changes ship with a runnable example in `examples/`. Pertinent means a new public entry point, a new tariff rule, or a new error variant. Examples must compile: `make lint` builds them through `clippy --all-targets`. Agents read `examples/` before calling an API. Why: an example shows the call in a few lines, faster than reading signatures and tests.

## Tax parameters

Tax values live in `rules/<year>.toml`. The build checks them and compiles them in, so the library never reads them at runtime. Why and how: [How fiskus is built](../internal/build/README.md).

Each year file has the basic allowance, `[basic_allowance]`, with the four fields `value_eur`, `citation`, `source` and `retrieved`, plus `reference`. A year with a tariff also has a `[source]` table with the same four fields, and one `[[zone]]` table per zone. A year without a tariff has neither.

A year with a free threshold for the solidarity surcharge (§ 3 Abs. 3 SolZG) also has a `[solidarity]` table, with `free_threshold_single_eur`, `free_threshold_joint_eur`, and the four source fields. A year without it has no solidarity figure, and `solidarity_surcharge` returns `None` for it.

The build fails on a file that:

- has a field other than the ones listed above, or a zone field other than `from_eur`, `base_eur`, `step_eur`, `quadratic_h`, `linear_h` and `constant_h`
- has a `value_eur` that is negative, or a blank `citation`, `source`, `retrieved` or `reference`
- has zones without a `[source]` table, or a `[source]` table without zones
- has a `[solidarity]` table with a negative threshold, or a joint threshold below the single one
- has a first zone that does not start at the allowance plus one euro, zones that are not in increasing order, a step that is not positive, or a tax that jumps by more than one euro at a zone start

The build does not read the `reference` file: it is a link on the GitLab main branch, so the packaged crate needs no local copy. `scripts/guide/checks.rs` checks that each reference names a file in `docs/references/`.

Tests do not hold the figures of a year. They read the rule data through the API, and run for every year in `TaxYear::ALL`. A figure is checked against its source when the file is written, and against the BMF calculator by the conformance layer.

| Change | Steps |
|---|---|
| New basic allowance value or zone for an existing year | 1. Change the value, the coefficients and the citation in `rules/<year>.toml`. 2. Check the change against the source, and the references README. 3. Update the guide and formula pages that quote it, if they quote it. `make test-guide` shows which. |
| New tax year | 1. Create `rules/<year>.toml` with `[basic_allowance]`, and `[source]` and `[[zone]]` if the tariff is known. 2. Add the year to `YEARS` in `build.rs`. 3. Add a variant to `TaxYear` and to `ALL`, and the `year()` arm. The compiler checks the rest. 4. Update the year count in `tests/public_api.rs` only if a test pins it. 5. Update the docs that state the range, and add the year's sources to `docs/references/README.md`. |

Keep `/build.rs` and `/rules/**/*.toml` in the `include` list of `Cargo.toml`, or the packaged crate cannot build. Check the list with `cargo package --list`. `docs/references` is not packaged, and the build does not read it.
