# How fiskus is tested

A tax figure is only useful if there is a reason to trust it. fiskus builds that reason in layers. Each layer asks a different question, costs a different amount to build and run, and catches a different kind of fault. Together they form a harness that is applied to every change and keeps growing as the library does.

This page explains what each layer is for, the theory behind it and how it applies to fiskus. The [further reading](#further-reading) at the end points to the research behind these methods.

{{#include ../assets/illustrations/test-pyramid.svg}}

The arrow on the left shows the cost to run, which rises towards the top. The base is wide and cheap. Static checks and unit tests are cheap and stateless, so they run on every change, as often as needed, because they are both inexpensive and valuable. The top is narrow and expensive. The upper layers take real effort to build and run, so they run less often. They cover only the specific things that must be tested, not every case. The dashed band is planned and not yet in fiskus.

## <span class="section-icon" style="-webkit-mask-image: url('../assets/icons/build.svg'); mask-image: url('../assets/icons/build.svg')"></span>Static checks

**What it does.** The compiler and the linters reject code that cannot be right before any test runs: type errors, unused or unreachable code, ignored warnings, and unsafe code, which fiskus forbids.

**In theory.** A type system is a small proof system. In safe Rust, the type and ownership rules rule out whole classes of faults, such as invalid memory access and data races, in every program that compiles.

**In practice.** Money is stored as whole cents in a 128-bit integer, never as a floating-point number, so rounding is exact and equality is reliable. The build also checks the tax parameters before any code runs against them: a negative value, a blank citation or a fractional euro amount stops the build. See [How fiskus is built](../build/README.md).

**Limit.** Static checks cannot tell whether a value is correct. They stop obviously wrong code from reaching a release.

## <span class="section-icon" style="-webkit-mask-image: url('../assets/icons/test.svg'); mask-image: url('../assets/icons/test.svg')"></span>Unit tests

**What it does.** A unit test names one case and asserts its exact value or error. Written beside the code, unit tests document what each part is meant to do, and they guide how a feature is built.

**In theory.** A unit test is a specification by example. The cases that matter most sit at the boundaries of a rule: the last value inside it, the first value outside, and the cents on either side of each.

**In practice.** Unit tests run the pure functions on fixtures: small tariffs with sample numbers, written in the test. They pin the rules that hold for any tariff, such as the zero zone, the rounding of one zone, and the overflow path. They do not pin the figures of a year. A figure from a statute belongs to the conformance layer, not to a unit test. A unit test covers only the cases someone thought of.

## <span class="section-icon" style="-webkit-mask-image: url('../assets/icons/wrench.svg'); mask-image: url('../assets/icons/wrench.svg')"></span>Public API tests

**What it does.** A public API test uses only what a user can import, and checks that each exported type behaves as documented: which inputs are accepted, which errors come back, what the error text says and how conversions round.

**In theory.** This is black-box testing. The public surface is the contract, so the tests use it as a user would. They split each input into equivalence classes and test the boundaries between them.

**In practice.** The public API tests use `fiskus` only, for every listed year, and take their expected values from the API itself: the basic allowance, the zones, the error variants. They check the contract around the numbers: each error path a user can reach, the messages, the conversions of `Cents`, and that `from_year` round-trips. No year and no amount is fixed in them.

## <span class="section-icon" style="-webkit-mask-image: url('../assets/icons/dices.svg'); mask-image: url('../assets/icons/dices.svg')"></span>Property tests

**What it does.** A property test states a rule that must hold for every input, then lets the test runner generate inputs until it finds a counterexample. The rules concern ranges and extremes: any amount in the zone is tax free, no integer makes the code panic, rounding always goes down.

**In theory.** Property testing comes from QuickCheck, which asked for properties instead of examples. When a counterexample is found, the runner shrinks it to a minimal failing input, which is usually the easiest bug to understand.

**In practice.** The generators reach the values a person would not write down: the largest amounts, odd cent values, the first value past each threshold. Random search can find a bug that a fixed list of examples misses. A passing run means no counterexample was found among the inputs tried, not that none exists. Failing inputs are saved and replayed, so a bug found once stays under test. The properties run over every listed year, using the rule data: the zones join, tax never falls as income rises, and joint tax is twice the tax on half the income.

## <span class="section-icon" style="-webkit-mask-image: url('../assets/icons/sigma.svg'); mask-image: url('../assets/icons/sigma.svg')"></span>Kani proofs

**What it does.** Kani checks Rust code against every input in a stated bound. If a property fails, it returns a concrete counterexample. If it holds, the result covers every input in the bound, not a sample.

**In theory.** Kani is a bounded model checker. It turns the Rust code into a logical model and asks a solver whether any input breaks the property. Inside the bound, the answer is exact. Kani's own documentation describes its proof harnesses as similar to property-based test harnesses, with the difference that they cover all inputs. A wider bound is a stronger claim, so the bounds can grow over time.

**In practice.** fiskus uses Kani for three things: each pure formula does exactly what the formula says, for every input in its bound; the arithmetic is checked, so an overflow is an error and never a wrong tax; and the types behave consistently at their edges, such as money that saturates instead of wrapping. The proofs are in `src/` next to the code they check, and each states its bound in a comment. A proof is dropped where the solver cannot finish it: the floor of a 128-bit fraction is a property test instead. Kani does not prove panic-freedom over the full `i128` range: that comes from the checked arithmetic, which the unit tests exercise with the largest values.

A proof does not extrapolate from examples. Checking ten thousand inputs and checking the whole bound are different claims, and only the second one is a proof. Proofs suit small functions with a clear boundary. Where a rule is too large for a proof to finish, property tests carry it.

## <span class="section-icon" style="-webkit-mask-image: url('../assets/icons/scale.svg'); mask-image: url('../assets/icons/scale.svg')"></span>Conformance

**What it does.** Conformance checks the data. It compares fiskus with results from the income tax calculator (Einkommensteuerrechner) that the Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) publishes. Each euro year option from 2002 to 2026 has 500 cases, across each year's income ranges, and each tax year that an option covers is checked against them. fiskus must agree with each result to the cent, or differ only in the two open problems listed in the test. `make gates` runs it, and a failing case blocks the gate.

**In theory.** This is differential testing: two independent implementations receive the same input, and any difference marks a bug in one of them. The BMF calculator is the reference for the data, so a match means fiskus gives the same answer as the official calculator for those inputs.

**In practice.** The expected values are the BMF-provided results, never fiskus's own output. The cases are committed, so the test runs offline. It runs as one test, and prints a scoreboard with a row for each tax year. A sample cannot cover every income, and conformance does not claim that it does. Kani covers the formulas across their bounds, and conformance covers the data. The tariff zones for 2003 to 2026 are implemented. Two cases in 2019 and 2020 differ by one to two euros. The BMF calculator computes in floating point, and a double-precision model of its formula reproduces both values. They are open problems, listed in `OPEN_PROBLEMS` in `tests/conformance/main.rs`.

## <span class="section-icon" style="-webkit-mask-image: url('../assets/icons/ruler.svg'); mask-image: url('../assets/icons/ruler.svg')"></span>Coverage

**What it does.** Coverage records which lines the tests run. The build fails below 90% line coverage.

**In theory.** Coverage is necessary but not sufficient. A line can run and still go unchecked, because a test executed it without asserting anything about the result. Mutation testing, which plants deliberate bugs to see whether the tests notice them, is the complementary check. It is not yet in fiskus.

**In practice.** The 90% floor stops untested code from growing quietly. It says nothing about whether the assertions are right.

## Ideas from FoundationDB

FoundationDB's testing rests on deterministic simulation. A whole cluster runs inside one process, time advances in steps, and every run can be repeated exactly from its seed. Its team runs tens of thousands of simulations every night and deliberately stresses the rare failure modes. fiskus is a single-threaded pure function with no network or disks to simulate, so most of that machinery does not apply. Four of its ideas do:

- **Every failure can be replayed.** A failing run is kept with its seed, so it can be studied rather than guessed at.
- **Stress the edges on purpose.** Generators favour the values where bugs hide: the zone thresholds, one cent on either side of them, zero and the largest values.
- **Run long, with fresh seeds.** Many more random runs than a single change needs, repeated over time, find the bugs that short runs miss. This is planned.
- **Keep a permanent record.** Every bug found becomes a stored case, so each change is judged against the history.

## How the layers combine

The layers run cheapest first. Static checks fail fastest, unit and API tests follow, and the expensive layers run last or on a schedule. Each layer covers a gap the one below cannot close. Unit tests cannot find what nobody wrote down. Property tests cannot prove that a fault is absent. Proofs cannot tell whether a formula is the law. Conformance cannot cover every income. Used together, most faults are caught by more than one layer.

A bug moves up the layers until something catches it. A property test finds a counterexample and shrinks it to a minimal input. The fix comes with a unit test that names the case, and the failing input is kept for replay. Once a bug is recorded at the lowest layer that can see it, the harness should catch it if it returns.

This is how the library is built stone by stone. Each change is added only when the harness passes, and the stones stay in place. A release is trustworthy because it passes the same harness as every change before it, and the harness only grows. That is also what makes fiskus easy to operate: the people running it do not need to re-check the library, because its history of checks is already on record.

## Limits

- The largest risk is the reading of the statute. The tests check consistency with `rules/`. Conformance checks the data against BMF results, but it does not replace that reading.
- Kani proofs hold inside their bounds.
- Property tests sample, and unit tests check named cases.
- Coverage shows what ran, not what is right.
- A figure from a draft, such as a year whose enacted text is not archived, is marked as a draft in its citation. It is not checked against enacted text until that text is archived.


## Further reading

| Topic | Source | Why read it |
|---|---|---|
| Bounded model checking of Rust | [Kani documentation](https://model-checking.github.io/kani/) | How proof harnesses turn Rust code into proofs or counterexamples |
| Model checking | [Wikipedia: model checking](https://en.wikipedia.org/wiki/Model_checking) | The general method behind Kani |
| Property-based testing, in practice | [The proptest book](https://altsysrq.github.io/proptest-book/proptest/getting-started.html) | A worked example, from a failing input to the fix |
| Property-based testing, origin | [Claessen and Hughes, QuickCheck (ICFP 2000)](https://www.cs.tufts.edu/~nr/cs257/archive/john-hughes/quick.pdf) | The original idea of properties and shrinking |
| Property testing | [Wikipedia: property testing](https://en.wikipedia.org/wiki/Property_testing) | An overview of the concept |
| Deterministic simulation | [FoundationDB: simulation and testing](https://apple.github.io/foundationdb/testing.html) | Seeded, repeatable runs, injected faults, and testing at scale |
| The system FoundationDB was built to test | [FoundationDB, SIGMOD 2021 (PDF)](https://www.foundationdb.org/files/fdb-paper.pdf) | The design behind the simulation |
| Differential testing | [Wikipedia: differential testing](https://en.wikipedia.org/wiki/Differential_testing), citing McKeeman (1998) | Two implementations, one input, and any difference is a bug |
| Conformance testing | [Wikipedia: conformance testing](https://en.wikipedia.org/wiki/Conformance_testing) | Checking an implementation against a specification |
| Fuzzing | [The Rust Fuzz Book](https://rust-fuzz.github.io/book/) | Coverage-guided fuzzing, not used in fiskus |
| Mutation testing | [cargo-mutants](https://mutants.rs/) | Planting bugs to test the tests |
| Coverage | [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) | Line coverage for Rust |
| Static analysis | [Clippy](https://doc.rust-lang.org/clippy/) | The lints behind the static checks |
