# How fast is fiskus

fiskus is not official software of the Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) or ELSTER, and nothing here is tax advice.

A tax engine has to be correct first. After that, speed decides whether it is useful. People run fiskus in batches and in tables of what-if scenarios, and each of those workflows has a speed it needs. Benchmarks show that speed. They do not show that a figure is correct, and they never block a change.

For how correctness is checked, see [How fiskus is tested](testing/README.md).

## What a benchmark measures

**What it does.** A benchmark runs one public call many times on fixed inputs and reports how many calls complete per second. The result is a throughput, not a single timing.

**In theory.** A single timing is noisy. A benchmark repeats the call, measures the spread of the results, and reports a figure with its uncertainty. The compiler can also remove work it sees is unused, so the inputs and outputs are passed through a special function that tells the compiler to keep them. Without that, a benchmark can measure nothing at all.

**In practice.** Inputs come from the statute boundaries and from values inside each zone, not from fiskus's own outputs. The inputs stay fixed, so each run is comparable with the last one. Cases are grouped under the public function they measure.

## Why measure over time

A single figure tells little. A series of dated figures, on a known machine, tells whether the calculation path got faster or slower. This is the same idea as regression testing: a change is judged against the history of the numbers, not against a memory of how fast the code once felt.

FoundationDB runs its performance tests every night and compares each day with the days before, so a slowdown shows up as a trend rather than as a surprise ([FoundationDB, performance testing](https://apple.github.io/foundationdb/testing.html)). fiskus follows the same idea at a smaller scale.

## Where the figures come from

A scheduled CI run executes the same benchmark every night on GitHub's Linux runners and publishes the throughput in the run's job summary. The series keeps growing without anyone having to remember to run it. Those runners are shared, so their figures show a trend. They are not results, and they do not replace the figures below.

## Coverage today

fiskus benchmarks the calculation paths that users call: `assess` and `explain` for every listed year, both filing statuses and the inputs that matter for the tariff, the solidarity surcharge, and the error path for a negative income. The inputs come from the rule data, so a new year is benchmarked without an edit: the zero-tax point, the allowance, twice the allowance, and every zone start of that year. Each figure says which inputs it uses.

## Current figures

Measured on 2026-11-25 on an Apple M2 Pro (12 cores, 32 GB memory, macOS 26.6.2) with Rust 1.100.0, using one thread and a release build. The batch of `assess` and `explain` covers 322 calls: for every listed year, the incomes 0, the allowance and twice the allowance, and each zone start, for single and joint filing.

| Case | Calls per second | Notes |
|---|---|---|
| `assess`, every listed year and zone boundary | about 42 million | 322 calls per batch |
| `explain`, same inputs | about 46 million | 322 calls per batch, with the rates and the citation |
| `solidarity_surcharge`, year and filing at each zone start | about 446 million | 178 calls per batch: 89 zone starts, for both filings. The years without a threshold return `None` at once |
| `assess`, negative income (error path) | about 276 million | One call per batch |
| Peak memory footprint while the benchmark ran | not measured | Not measured in this run. Measure it before quoting a figure |

## What the figures do not say

- They describe one machine and one thread. They are not a promise for your hardware, and they will change as the calculation grows.
- The figures change with the number of listed years: a new year adds calls to each batch, so compare batches of the same size only.
- fiskus has not verified that the calculation makes zero heap allocations per call, so that claim is not made.
- Memory has been measured only on macOS.
- A browser build is not yet in fiskus, so it has not been measured.

## What comes next

fiskus must keep its performance steady while it grows. Each new zone, filing path and tax rule adds work to the calculation, and that cost should be seen before it reaches a release rather than after. Changes with nothing to do with tax rules matter too. A new Rust toolchain, an updated dependency or a refactor can slow a hot path without anyone meaning to.

The plan is to keep the benchmark running over time, add the new paths as they land, and record each change in the figures. Additions and removals can then be checked against the numbers that came before them.

Linux and Windows figures are planned. Both platforms matter to people who run fiskus. Getting their figures right takes more work than the macOS numbers, because the machines, the memory measurements and the tooling all differ. They will be added carefully rather than rushed.

## Further reading

| Topic | Source | Why read it |
|---|---|---|
| Statistical benchmarking in Rust | [The Criterion.rs book](https://bheisler.github.io/criterion.rs/book/) | The tool fiskus uses, with its history and reports |
| How the results are analysed | [Criterion.rs: analysis process](https://bheisler.github.io/criterion.rs/book/analysis.html) | Why a single run is not a result |
| Preventing the compiler from removing work | [`std::hint::black_box`](https://doc.rust-lang.org/std/hint/fn.black_box.html) | Why inputs and outputs are kept opaque to the optimiser |
| Benchmarks in general | [Wikipedia: benchmark (computing)](https://en.wikipedia.org/wiki/Benchmark_(computing)) | Background on what a benchmark can and cannot show |
| Throughput | [Wikipedia: throughput](https://en.wikipedia.org/wiki/Throughput) | The unit used for every figure on this page |
| Performance testing | [Wikipedia: performance testing](https://en.wikipedia.org/wiki/Performance_testing) | How performance is tested alongside correctness |
| Regression testing | [Wikipedia: regression testing](https://en.wikipedia.org/wiki/Regression_testing) | Why a series of figures catches slowdowns |
| Nightly performance testing at scale | [FoundationDB: performance testing with Circus](https://apple.github.io/foundationdb/testing.html) | A daily comparison of each build with the days before |
