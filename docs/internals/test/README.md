{{#include ../assets/icons/test.svg}}

# How we test

Every tax figure fiskus returns should be trustworthy for a reason we can point to. This page describes how the testing supports that: which kinds of evidence we collect, what each one can and cannot show, and where the limits are.

For how the build turns the figures into code, see [How the build fits together](../build/).

## Why several kinds of evidence

No single check proves a tax rule is right. A unit test shows one case. A property test shows a rule holding on many random inputs. A proof shows it holding on every input in a bounded range. A docs check shows that a claim in the guide still matches the code. Each answers a different question, so fiskus uses several of them, and each page says what its check leaves out.

{{#include ../assets/illustrations/test-pyramid.svg}}

The pyramid shows how the layers relate. The base is wide and cheap: many checks that run on every change. The top is narrow and expensive: a few proofs, kept for the rules where a mistake would cost most. The shape shows the idea, not a count of tests.

## What each layer answers

| Layer | Question it answers | Tool | What it cannot show |
|---|---|---|---|
| Static checks | Is the code well-formed, and is the parameter data valid? | `make check`: rustfmt, clippy, `build.rs` | Whether a value is correct |
| Unit tests | Does a named case give the expected value or error? | `make test` (`cargo test --lib`) | Cases nobody wrote down |
| Doc tests | Do the examples in the docs still compile and give the stated answer? | `make check` (`cargo test --doc`) | Anything outside the examples |
| Guide and chart checks | Do the guide and its charts still match the code and `rules/`? | `make check`: `scripts/guide/checks.rs`, `charts.rs --check` | Whether an explanation is correct, only that it is consistent |
| Property tests | Does a rule hold for any input in a range? | `make test` (proptest) | A guarantee. Random sampling finds bugs but does not rule them out |
| Kani proofs | Does a rule hold for every input in a bound, and never panic? | `make verify` (Kani) | Inputs beyond the bound |

## Static checks: the cheapest evidence

Rust catches a lot before any test runs. Money is stored as whole cents in `i128`, never as floating point, so rounding is exact and comparisons are reliable. rustfmt keeps the code uniform, which makes review easier. clippy runs with warnings treated as errors, so a suspicious pattern cannot pass quietly.

The build adds a data-level check. It rejects a parameter that is negative, has a blank citation, or is not a whole euro amount, before any code runs against it.

## Unit tests and doc tests

Each unit test covers one case, and the comment above it says why that case matters. For a tax zone, the cases that matter most are the boundaries: the last amount inside it, the first amount outside, and cents on either side. The examples in the public docs are compiled and run as doc tests, so the documentation cannot show a call that no longer works.

Here is one case from `src/tariff.rs`:

```rust
// Joint filer at 24 698 €: the half is 12 349 €, whose tax rounds down to 0 €.
#[test]
fn joint_one_euro_above_the_split_zone_is_still_tax_free() {
```

## Property tests

A property test states a rule and lets the test runner try many inputs. For example, any amount in the zone, with any cents, must give no tax. It is a strong check across a range, but it samples. A passing run means no counterexample was found among the inputs tried.

## Proofs

Kani checks a function against every input in a bound, rather than against a sample. Five proofs run today:

- `zero_zone_is_exact` in `src/tariff.rs`: up to the basic allowance the tariff is zero, for any allowance and income within ±10^12 euros
- `tariff_never_panics` in `src/tariff.rs`: for any allowance within the bound and any income that `Cents` can hold, the VZ 2026 tariff returns a non-negative tax without overflow
- `filing_basis_non_negative` in `src/tariff.rs`: the tariff basis is non-negative for non-negative taxable income, single and joint
- `from_euros_saturates_instead_of_overflowing` in `src/money.rs`
- `whole_euros_is_floor_division` in `src/money.rs`: `whole_euros` equals floor division by 100 for every cents value

The bound is part of the claim. The zero-zone proof covers euros within ±10^12, which is 1 trillion euros. The no-panic proof covers every income `Cents` can hold, so the tariff is checked for any input at all. Proven functions take their parameters as arguments, so a proof never has to read the TOML.

Proofs suit small functions with a clear boundary. Where a rule is too large for a proof to finish, the property tests carry it.

## Docs are checked too

The guide makes claims about figures, citations and charts, and tests compare those claims with the code and `rules/`:

- a guide page may only cite the statute sections listed for it
- the figures on a page match the parameter values
- committed charts match what the code draws
- the internals pages name every Kani proof, and any `make` target they mention exists

These checks do not judge whether an explanation is good. They stop it from drifting away from the code it describes.

## Where the tax figures come from

Tests can show that the code does what the figures say. They cannot show that the figures match the law. That comparison is a human reading. Each value in `rules/` carries its citation, its source and the date it was retrieved, so a reviewer can check it against the statute. The tests make the data complete and consistent, and they fail when an entry is missing a part.

## How the layers combine

{{#include ../assets/illustrations/test-gates.svg}}

The gates run in order, cheapest first, so a broken change fails early. While changing code, `make test` gives the quick loop. `make check` runs every other check, and `make verify` runs the proofs. A change that touches a proof also runs the proofs. CI runs the same gates on every push and pull request, so the result seen locally is the result the repository gets.

Changes are handed over with their gate results recorded, and a commit is made once the gates pass.

## Tools

| Tool | Role | Pinned in |
|---|---|---|
| Rust | Language and toolchain | `mise.toml`, `rust = 1.100.0` |
| rustfmt | Formatting | `mise.toml` |
| clippy | Lints, with warnings treated as errors | `mise.toml` |
| Cargo | Builds and runs every test target | `Cargo.toml` |
| proptest | Property tests | `Cargo.toml`, dev-dependencies |
| Kani | Proofs over bounded inputs | `mise.toml`, `kani-verifier 0.68.0` |
| mdBook and KaTeX | Builds this book with `make docs` | `mise.toml`, `mdbook 0.5.3`, `mdbook-katex 0.10.0` |
| mise | Installs the versions above | `mise.toml` |
| GitHub Actions | Runs the gates on every push and pull request | `.github/workflows/ci.yml` |

## What we do not claim

- The tariff is implemented for every year in `TaxYear`, VZ 2006 to 2026. Years before 2006 are not in `TaxYear`.
- Every year's coefficients are checked against the text of its BMF withholding guidance (PAP), and every zone join is checked for continuity. The unit tests compare five incomes per year with an independent calculation. A comparison of every whole euro from 0 to 300 000 € was run for VZ 2026 only, outside the repository.
- Proofs hold inside their bounds. Property tests sample, and unit tests check named cases.
- The tests check consistency with `rules/`, not the statute itself. See [where the figures come from](#where-the-tax-figures-come-from).
