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
/// and [`Error::OutsideImplementedRange`] above the implemented zone.
pub fn assess(year: TaxYear, unit: &FilingUnit) -> Result<Assessment, Error>
```

Bad: `/// Assesses tax.`

- Doc examples go in a `/// # Examples` section, run by `make test-doc`. Use only the public API, not `crate::` paths. Why: doc examples show the public call shape, so they must compile against what users import.
- Pertinent changes ship with a runnable example in `examples/`. Pertinent means a new public entry point, a new tariff rule, or a new error variant. Examples must compile: `make lint` builds them through `clippy --all-targets`. Agents read `examples/` before calling an API. Why: an example shows the call in a few lines, faster than reading signatures and tests.

## Tax parameters

Tax values live in `rules/<year>.toml`. The build checks them and compiles them in, so the library never reads them at runtime. Why and how: [How fiskus is built](../internal/build/README.md).

The build fails on an entry that:

- has a field other than `value_eur`, `citation`, `source`, `retrieved` or `reference`
- has a `value_eur` that is negative or not a whole number
- has a blank `citation`, `source`, `retrieved` or `reference`
- has a key that is not lowercase snake_case, since keys become Rust field names

| Change | Steps |
|---|---|
| New parameter | 1. Add a table with the five fields (`value_eur`, `citation`, `source`, `retrieved`, `reference`) to the year's file. 2. Add a `Parameter` field with the same name to `Parameters` in `src/params.rs`. Without it the generated code does not compile. 3. Read it through the year's lookup. An unread field fails `make lint`. |
| New tax year | 1. Create `rules/<year>.toml`. 2. Add the year to `YEARS` in `build.rs`. 3. Add a variant to `TaxYear`. 4. Add a match arm in the lookup in `src/params.rs`. The compiler checks steps 3 and 4. Document the year on the usage page. |

Keep `/build.rs` and `/rules/**/*.toml` in the `include` list of `Cargo.toml`, or the packaged crate cannot build. Check the list with `cargo package --list`.
