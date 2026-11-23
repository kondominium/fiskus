# Code gates and rules

Applies to humans and agents.

## Gates

- Scratch: research, drafts and plans go in `.agents/research/` and `.agents/plans/`. Both are git-ignored and never committed.
- Commit: agents never commit. Hand over a staged diff (only the slice's files), a commit message draft, and the `make check` result.
- Check: `make check` must pass before handover.
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

- Doc examples go in a `/// # Examples` section, run by `cargo test --doc`. Use only the public API, not `crate::` paths. Why: doc examples show the public call shape, so they must compile against what users import.
- Pertinent changes ship with a runnable example in `examples/`. Pertinent means a new public entry point, a new tariff rule, or a new error variant. Examples must compile: `make check` builds them through `clippy --all-targets`. Agents read `examples/` before calling an API. Why: an example shows the call in a few lines, faster than reading signatures and tests.

## Tests

This section says which test layer proves a claim, and when to add tests.

### Pick the layer that proves the claim

- One value or error for one case: a unit test beside the code, in `#[cfg(test)] mod tests`.
- A property that must hold for many inputs (no panic, rounding, zone membership): `proptest` in the same module.
- A pure function that must hold over a bounded domain or all inputs: a Kani harness in `#[cfg(kani)] mod verification`. Pass parameters in, so Kani never parses TOML.
- What users import (conversions, error text, the public call shape): a black-box test in `tests/`, using only `fiskus::` paths.
- Doc claims and examples: a doc test on the public item. A new entry point, tariff rule or error variant also gets a runnable example.
- Guide and code agreeing: `scripts/guide/checks.rs`. Extend it when a new rule needs a check. Do not work around it.

Test a claim in one layer. Repeat it in another only when the second layer checks something different.

### When to add what

| Change | Add |
|---|---|
| New tariff rule or zone | A formula page in `docs/formulas/` first. Unit tests on both sides of each boundary, with cents. One proptest for the rule's property. A Kani harness if the function is pure. A public test if users see it. A guide update. |
| New public item | A `///` doc with units and errors, and a doc example. A `tests/` test that calls it. |
| New error variant | A test for its `Display` text, a test that returns it, and an example. |
| Bug fix | A failing test that reproduces the bug, committed with the fix. |
| Parameter value change | The `rules/` citation and source, the formula page and the guide. Update every test that cites the value, with the same citation. |
| Refactor with no behaviour change | Nothing new. The existing tests must pass. Do not add tests that only pin internals. |
| Docs only | The guide checks. New tests only if a claim changed. |

### How many

- For each threshold, test the value just below, at, and just above it, with cents where they matter. Add one interior value. That is the whole table, not a grid.
- Keep proptest to one or two properties per rule. Use a table of cases where a few examples say it clearly.
- Write Kani harnesses only for pure functions, and state the bound in a comment.

### Good and bad patterns

Each test has a one-line `//` comment above it that names the case and cites its section. Why: the test name says what runs, the comment says which case and why it matters.

Good:

```rust
// Joint filer at 24 698 €: the half is 12 349 €, just outside the zone (§ 32a Abs. 5 EStG).
#[test]
fn joint_one_euro_above_the_split_zone_is_outside_the_implemented_range() {
    let error = assess(TaxYear::Y2026, &joint((JOINT_ZONE_EUR + 1) * 100)).unwrap_err();
    assert_eq!(error, Error::OutsideImplementedRange { taxable_income_eur: JOINT_ZONE_EUR + 1 });
}
```

Bad: `assert!(result.is_ok());` on its own. It passes for any amount. Assert the value.

Bad: expected values copied from fiskus's own output. Take them from the statute, or from the BMF calculator for conformance tests. A value copied from our output tests nothing.

Bad: a proptest that re-implements the rule to compare against. It repeats the same mistake. Assert a property instead: zone membership, rounding direction, no panic.

Bad: one test that covers several zones and several cases. Split it, so a failure names the case.

Bad: `#[ignore]` without a linked issue. Ignored tests hide gaps.

Bad: a test that reads a file or TOML. The library reads no files at runtime, and tests should not hide a dependency on one.

Bad: asserting on `Debug` output. Assert values and `Display` text, which users see.

### Gates

- `make check` runs fmt, clippy with warnings denied, and the unit, proptest, doc, public API and guide tests.
- `make verify` runs the Kani harnesses. The `verify` CI job runs it on every push and pull request.
- `make coverage` writes a line coverage report to `target/llvm-cov/lcov.info` and fails below 90% line coverage. The `coverage` CI job runs it on every push, blocks the build on failure, and uploads the file as an artifact. The threshold can be overridden with `COVERAGE_MIN_LINES` in the environment. The excluded paths (`scripts/`, `build.rs`) are set in `COVERAGE_IGNORE_REGEX` in `mise.toml`.
- Kani needs bounded integers to finish. The boundary harness bounds cents to ±10^12 (see the comment in `src/tariff.rs`). The no-panic harness covers the full `i128` range.
- Each tariff rule has a page in `docs/formulas/` first, and its tests cite the same section.

## Benchmarks

Benchmarks in `benches/` measure the calculation path for speed. They assert nothing and do not gate a change. Run them with `make bench`. The reasons for publishing the figures are in `docs/internal/benchmarks.md`.

### When to add one

- A new public function on the calculation path, or a public call that users may run in bulk.
- A new zone. Add its boundary inputs to the existing `assess` group, not a new function per zone.
- A change that rewrites the hot path (zone check, parameter lookup, rounding, error construction) and needs evidence that it did not get slower.

### When not to add one

- Docs, guide, error text or tooling changes.
- Code off the calculation path, or private helpers. Benchmark the public call that uses them.
- To justify an optimisation before there is a measured problem. Measure first, then benchmark that case.
- To give a number we have no reason to publish. Every benchmark is a number someone may quote.
- To put timings in CI. Shared runners are too noisy, and a timing gate would flake.

### How to write one

- Use `Throughput::Elements`, so the output is calls per second.
- Wrap inputs and outputs in `black_box`, so the compiler cannot skip the call.
- Take inputs from the statute boundaries and interior values, not from fiskus's own outputs.
- Keep inputs fixed, so runs are comparable.
- Group cases under the public function's name.
- Benchmarks compile under `make check`, since clippy runs with `--all-targets`. A broken benchmark fails the gate even though `make check` does not run it.

### Running and recording

- `make bench` writes the criterion report to `target/criterion`. Criterion prints the change against the last run, so compare before and after a hot-path change.
- A change that moves the hot path also updates the Performance section of `README.md`: the machine, the date and the figures, as the ADR describes.
- Peak memory uses macOS `time -l`. Build the bench binary with `mise exec -- cargo bench --bench tariff --no-run`, and the last line of that output names the executable. Run `/usr/bin/time -l <executable> --bench --profile-time 10` and read the **peak memory footprint** line. Do not publish the **maximum resident set size** line, since it counts shared system libraries.

## Commit messages

- Title: imperative mood, verb first, 72 characters or fewer, no type prefix. Examples: `Pande viam fisci`, `Bootstrap repo fundamentals`.
- Body: English. Say what changed and why. The diff already shows how. Wrap at 72 characters.
- Scope: one slice per commit. Stage only that slice's files.
- Gate: `make check` passes before the commit. Add `make verify` when a Kani harness changes.
- Agents never commit. An agent hands over the staged files, the message draft, and the gate result. The owner commits.
- The `commit-message` skill (`.agents/skills/commit-message.md`) follows this section.
