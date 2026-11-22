# Code gates and rules

Applies to humans and agents.

## Gates

- Scratch: research, drafts and plans go in `.agents/research/` and `.agents/plans/`. Both are git-ignored and never committed.
- Commit: agents never commit. Hand over a staged diff (only the slice's files), a commit message draft, and the `make check` result.
- Check: `make check` must pass before handover. It runs every test, the lints and the guide chart check.
- Quick loop: `make test` runs only the unit and property tests in `src/`.
- Guide: a change that adds or alters a rule, parameter, zone or error updates `docs/guide/` in the same diff. See [guide](guide.md).

## Rules

- No `f64` for money: binary floats can't store decimals exactly (`0.1 + 0.2 != 0.3`), which breaks equality checks and rounding. Use integer cents or a decimal type.
- No `unwrap()` or `expect()` in library code reachable from user input. Return `Result`: bad input would panic the caller instead of returning an error it can handle.
- No runtime filesystem reads. Tax parameters are compiled in: the binary is self-contained and reproducible, and stray files can't silently change results.
- Provenance (the § citation and parameter version behind each number) is a planned separate feature. It is not in the code yet. The citations and sources stay in `rules/` as reference: readers shouldn't assume the code tracks sources it doesn't yet.
- Verify tax figures against the statute before relying on them: tax law changes, and numbers from memory or an LLM are often wrong.
- Identifiers are English. Types, functions, fields, variables, constants, files, test names and parameter keys never use German, even where German is the precise tax term: one language keeps code consistent and searchable.
- Docs and doc comments follow the [language rules](language.md), including German aliases in parentheses after the English term. Why: readers find the German term, while code keeps the English name.

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

- Doc examples go in a `/// # Examples` section, run by `make check` (`cargo test --doc`). Use only the public API, not `crate::` paths. Why: doc examples show the public call shape, so they must compile against what users import.
- Pertinent changes ship with a runnable example in `examples/`. Pertinent means a new public entry point, a new tariff rule, or a new error variant. Examples must compile: `make check` builds them through `clippy --all-targets`. Agents read `examples/` before calling an API. Why: an example shows the call in a few lines, faster than reading signatures and tests.

## Tests

- Unit tests sit beside the code in `#[cfg(test)] mod tests`, and check behaviour (values and errors), not declarations.
- Each test has a one-line `//` comment above it naming the case it covers, e.g. `// Joint filers exactly at the zone limit stay tax-free.` Why: the test name says what runs, the comment says which case and why it matters.
- Properties use `proptest` in the same module.
- Proof harnesses use Kani, in a `#[cfg(kani)] mod verification` beside the code. Keep the proven function pure: parameters are passed in, not loaded, so Kani never parses TOML.
- `make test` runs the unit and proptest tests in `src/`.
- `make check` runs every test (unit, property, guide and doc tests), plus fmt, clippy and the chart check, on every push and pull request.
- `make verify` runs only the Kani harnesses, in the `verify` CI job on every push and pull request.
- Kani needs integer ranges to finish. The boundary harness bounds cents to ±10^12 (see the comment in `src/tariff.rs`). The no-panic harness covers the full `i128` range.
- Each tariff rule has a page in `docs/formulas/` first, and its tests cite the same section.

## Commit messages

- Title: imperative mood, verb first, 72 characters or fewer, no type prefix. Examples: `Pande viam fisci`, `Bootstrap repo fundamentals`.
- Body: English. Say what changed and why. The diff already shows how. Wrap at 72 characters.
- Scope: one slice per commit. Stage only that slice's files.
- Gate: `make check` passes before the commit. Add `make verify` when a Kani harness changes.
- Agents never commit. An agent hands over the staged files, the message draft, and the gate result. The owner commits.
- The `commit-message` skill (`.agents/skills/commit-message.md`) follows this section.
