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

fiskus aims to benchmark every code path it has: each filing status, each tariff zone, and the error path for inputs it rejects. Today that is narrower than the goal. The benchmark covers both filing statuses at the edges of the basic allowance (Grundfreibetrag) zone, which is the only zone implemented so far, plus the error path. As each new zone lands, its boundary and interior cases will join the benchmark, and the published figures will say which zones they include.

## Current figures

Measured on 2026-11-23 on an Apple M2 Pro (12 cores, 32 GB memory, macOS 26.6.2) with Rust 1.100.0, using one thread and a release build.

| Case | Calls per second | Notes |
|---|---|---|
| `assess`, inside the implemented zone | about 199 million | The basic allowance zone |
| `assess`, outside the implemented zone (error path) | about 300 million | Inputs the calculator rejects |
| Peak memory footprint while the benchmark ran | 1.6 MB | Measured on macOS |

## What the figures do not say

- They describe one machine, one thread and one zone. They are not a promise for your hardware, and they will change as more of the tariff is implemented.
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
