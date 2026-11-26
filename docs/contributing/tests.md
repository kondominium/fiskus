# Tests and benchmarks

Applies to humans and agents. Each test answers one question. This page says which layer proves a claim and when to add one. Why each layer exists is in [How fiskus is tested](../internal/testing/README.md).

## Run tests with make

Run tests through their `make` target, never a raw `cargo test`, so the flags match CI.

| Target | Runs | Use it for |
|---|---|---|
| `make test-unit` | Unit and property tests in `src/` | The quick loop while coding |
| `make test-doc` | Doc tests | Doc examples |
| `make test-api` | `scripts/tests/public_api.rs` | What users import, errors, conversions |
| `make test-proofs` | Kani proofs in `src/` | Pure functions over a stated bound |
| `make test-all` | All of the above, except coverage | Before handover |
| `make test-coverage` | Line coverage, fails below 90% | Before handover |
| `make test-conformance` | Calculator conformance cases, per tax year, with a scoreboard | Part of `make test-all`, so `make gates` and CI. Every case matches, or is one of two open problems listed in the test (2019-0215 and 2020-0335) |
| `make bench` | Criterion benchmarks | Hot-path changes. Not a gate |

## The test pyramid

Each layer answers one question, and a test belongs to the lowest layer that can answer it. The table lists each layer with what it is for and what it must not do.

| Layer | Question it answers | Where | Must not |
|---|---|---|---|
| Unit | Does this function do what its rule says, on a small case? | `#[cfg(test)] mod tests` beside the code | Pin a figure of a real year |
| Property | Does the rule hold for every input in a range? | `proptest` in the same modules, and `tests/` for the public API | Pin a figure of a real year |
| Proof (Kani) | Does this pure function hold for every input in a bound? | `#[cfg(kani)] mod verification` | Be unbounded. Each proof states its bound |
| API | Does the public contract behave as documented? | `scripts/tests/public_api.rs`, `fiskus::` paths only | Use internal modules |
| Doc | Does each example compile and run? | `///` examples on public items | Show a figure that a change would silently break |
| Guide | Do the guide and the rules agree? | `scripts/guide/checks.rs` and the chart drift check | Be worked around. Extend it for a new rule |
| Conformance | Does fiskus match the BMF calculator for chosen incomes? | `scripts/conformance/`, fixtures in `tests/fixtures/conformance/`. Part of `make test-all` | Pin a figure in the other layers. Add cases only through the fetch scripts |

**Figures belong to conformance.** The unit, property, proof and API layers use the rule data and the properties of the tariff, never a fixed figure for a year. The tests run for every year in `TaxYear::ALL`, so a new year is covered without an edit. Fixed figures wait for the conformance layer, where they are checked against the BMF calculator.

Test a claim in one layer. Repeat it in another only when the second layer checks something different.

## Good and bad

Each test has a `//` comment above it that names the case and cites its section, when the case comes from the statute. The test name says what runs.

Good: a property over every listed year, with its rule named.

```rust
// Joint tax is exactly twice the tax on half of the income, rounded down (§ 32a Abs. 5 EStG).
proptest! {
    #[test]
    fn joint_tax_is_twice_the_single_tax_on_half_the_income(index in year_index(), euros in 0..2_000_000i128) {
        let year = TaxYear::ALL[index];
        // ...
    }
}
```

Bad: a fixed figure for one year, which breaks when the rules change for that year, and tests nothing else.

```rust
// A figure for one year, copied from a statute.
assert_eq!(assess(year, &joint(2_471_200)).unwrap().income_tax, Cents::from_euros(2));
```

## When to add what

| Change | Add |
|---|---|
| New tariff rule or zone | A page in `docs/formulas/` first. A property that the rule holds for every year. A Kani harness if the function is pure. A public test if users see it. A guide update. The fixed figures wait for conformance |
| New public item | A `///` doc with units and errors, a doc example, and a `tests/` test that calls it. An example in `examples/` if it is a new entry point |
| New error variant | A test for its `Display` text, a test that returns it, and an example |
| Bug fix | A failing test that reproduces the bug, committed with the fix |
| Parameter value change | The `rules/` citation and source, the formula page and the guide. The properties need no edit |
| Refactor with no behaviour change | Nothing new. The existing tests must pass. Do not add tests that only pin internals |
| Docs only | The guide checks. New tests only if a claim changed |

## How many

- Unit tests: the rule's boundary values, the value just below and just above, with cents where they matter, and one interior value, using fixture data.
- Property tests: one or two properties per rule. Keep the input ranges bounded, so a failing case is readable.
- Kani harnesses: only for pure functions, with the bound stated in a comment. Keep each bound small enough that the proof runs in CI.
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
