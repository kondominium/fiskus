# Tests and benchmarks

Applies to humans and agents. Each test answers one question. This page says which layer proves a claim and when to add one. Why each layer exists is in [How fiskus is tested](../internal/testing/README.md).

## Run tests with make

Run tests through their `make` target, never a raw `cargo test`, so the flags match CI.

| Target | Runs | Use it for |
|---|---|---|
| `make test-unit` | Unit and property tests in `src/` | The quick loop while coding |
| `make test-doc` | Doc tests | Doc examples |
| `make test-api` | `tests/public_api.rs` | What users import, errors, conversions |
| `make test-guide` | Guide checks and the chart drift check | `docs/guide/`, charts, icons |
| `make test-proofs` | Kani proofs | Pure functions over a bound |
| `make test-all` | All of the above, except coverage | Before handover |
| `make test-coverage` | Line coverage, fails below 90% | Before handover |
| `make bench` | Criterion benchmarks | Hot-path changes. Not a gate |

## Pick the layer that proves the claim

| Claim | Layer | Where |
|---|---|---|
| One value or error for one case | Unit test | `#[cfg(test)] mod tests` beside the code |
| Extremes, unexpected inputs and whole ranges: no panic, rounding, zone membership | Property test (`proptest`) | Same module as the rule |
| A formula holds for every input in a bound, and the code cannot panic | Kani harness | `#[cfg(kani)] mod verification` |
| Functionality of exported types: constructors, conversions, the public call and its error paths | Public API test | `tests/`, using `fiskus::` paths only |
| The BMF calculator's results for chosen incomes | Conformance test | Not yet in fiskus. Do not add fixtures until the conformance work lands |
| Claims in docs and examples | Doc test | `///` on the public item |
| Guide and code agree | Guide check | `scripts/guide/checks.rs`. Extend it for a new rule. Do not work around it |

Test a claim in one layer. Repeat it in another only when the second layer checks something different.

## When to add what

| Change | Add |
|---|---|
| New tariff rule or zone | A page in `docs/formulas/` first. Unit tests on both sides of each boundary, with cents. One proptest for the rule's property. A Kani harness if the function is pure. A public test if users see it. A guide update |
| New public item | A `///` doc with units and errors, a doc example, and a `tests/` test that calls it |
| New error variant | A test for its `Display` text, a test that returns it, and an example |
| Bug fix | A failing test that reproduces the bug, committed with the fix |
| Parameter value change | The `rules/` citation and source, the formula page and the guide. Update every test that cites the value, with the same citation |
| Refactor with no behaviour change | Nothing new. The existing tests must pass. Do not add tests that only pin internals |
| Docs only | The guide checks. New tests only if a claim changed |

## How many

- For each threshold, test the value just below, at and just above it, with cents where they matter. Add one interior value. That is the whole table, not a grid.
- Keep proptest to one or two properties per rule. Use a table of cases where a few examples say it clearly.
- Write Kani harnesses only for pure functions, and state the bound in a comment.
- Kani needs bounded integers to finish. The boundary harness bounds cents to ±10^12 (see the comment in `src/tariff.rs`). The no-panic harness covers the full `i128` range.
- Pass parameters into a harness, so a proof never parses TOML.

## Good and bad

Each test has a `//` comment above it that names the case and cites its section. The test name says what runs. The comment says which case and why it matters.

Good:

```rust
// Joint filer at 24 698 €: the half is 12 349 €, just outside the zone (§ 32a Abs. 5 EStG).
#[test]
fn joint_one_euro_above_the_split_zone_is_outside_the_implemented_range() {
    let error = assess(TaxYear::Y2026, &joint((JOINT_ZONE_EUR + 1) * 100)).unwrap_err();
    assert_eq!(error, Error::OutsideImplementedRange { taxable_income_eur: JOINT_ZONE_EUR + 1 });
}
```

Bad:

- `assert!(result.is_ok());` alone. It passes for any amount. Assert the value.
- Expected values copied from fiskus's own output. Take them from the statute. A value copied from fiskus's own output tests nothing.
- A proptest that re-implements the rule to compare against. It repeats the same mistake. Assert a property: zone membership, rounding direction, no panic.
- One test that covers several zones and several cases. Split it, so a failure names the case.
- `#[ignore]` without a linked issue. Ignored tests hide gaps.
- A test that reads a file or TOML. The library reads no files at runtime. The guide checks are the exception, because they compare docs with `rules/`.
- Asserting on `Debug` output. Assert values and `Display` text, which users see.

## Benchmarks

Benchmarks in `benches/` measure the calculation path for speed. They assert nothing and do not gate a change. The `nightly` workflow runs them and writes the throughput to the job summary. The published figures and the reasons for them are in [Benchmarks](../internal/benchmarks.md).

### When to add one

| Case | Benchmark? |
|---|---|
| New public function on the calculation path, or a public call users may run in bulk | Yes |
| New zone | Yes, its boundary inputs in the existing `assess` group, not a new function |
| Change that rewrites the hot path: zone check, parameter lookup, rounding, error construction | Yes, to show it did not get slower |
| Docs, guide, error text or tooling | No |
| Private helper | No. Benchmark the public call that uses it |
| Optimisation with no measured problem | No. Measure first |
| Timings in CI | No. Shared runners are too noisy and a timing gate would flake |

### How to write one

- Use `Throughput::Elements`, so the output is calls per second.
- Wrap inputs and outputs in `black_box`, so the compiler cannot skip the call.
- Take inputs from the statute boundaries and interior values, not from fiskus's own outputs.
- Keep inputs fixed, so runs are comparable.
- Group cases under the public function's name.
- Benchmarks compile under `make lint`, since clippy runs with `--all-targets`. A broken benchmark fails the lint.

### Running and recording

- `make bench` writes the criterion report to `target/criterion`. Criterion prints the change against the last run, so compare before and after a hot-path change.
- A change that moves the hot path also updates the figures in `docs/internal/benchmarks.md` and the Performance section of `README.md`. The two must agree.
- Peak memory uses macOS `time -l`. Build the bench binary with `mise exec -- cargo bench --bench tariff --no-run`. The last line of that output names the executable. Run `/usr/bin/time -l <executable> --bench --profile-time 10` and read the **peak memory footprint** line. Do not publish the **maximum resident set size** line, since it counts shared system libraries.
