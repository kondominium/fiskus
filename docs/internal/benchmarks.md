# Benchmarks

fiskus is not official BMF or ELSTER software, and nothing here is tax advice.

Performance matters to us. A tax engine has to be correct first, but people run it in batches, in tables of what-if scenarios and in the browser, and each of those workflows has a speed it needs. We want anyone using fiskus to be able to look at our numbers and judge whether it is fast enough for what they plan to do.

So we measure performance over time, not once. The benchmark in `benches/tariff.rs` runs the same calls on the same kind of input each time, and we record the results with the machine, the software versions and the date. A single figure tells you little. A series of dated figures on a known machine tells you whether the calculation path got faster or slower.

A scheduled CI run executes the same benchmark every night on GitHub's Linux runners and publishes the throughput in the run's job summary, so the series keeps growing without anyone having to remember to run it. Those runners are shared, so their figures show a trend. They are not results, and they do not replace the figures in the README.

We aim to cover every code path the calculator has: each filing status, each tariff zone, and the error path for inputs it rejects. Today that is narrower than the goal. The benchmark covers both filing statuses at the edges of the basic allowance zone, which is the only zone implemented so far, plus the error path. As each new zone lands, its boundary and interior cases will join the benchmark, and the published figures will say which zones they include.

The current figures were measured on 2026-11-23 on an Apple M2 Pro (12 cores, 32 GB memory, macOS 26.6.2) with Rust 1.100.0, using one thread and a release build. Inside the implemented zone, `assess` ran at about 199 million calls per second. On the error path it ran at about 300 million calls per second. The peak memory footprint while the benchmark ran was 1.6 MB.

These figures describe one machine, one thread and one zone. They are not a promise for your hardware, and they will change as more of the tariff is implemented. We have not verified that the calculation makes zero heap allocations per call, so we do not claim it. We have measured memory only on macOS, and the browser calculator has not been measured at all.

What comes next is keeping performance steady while the calculator grows. Each new zone, filing path and tax rule adds work to the calculation, and we want to see that cost before it reaches a release rather than after. The same goes for changes that have nothing to do with tax rules: a new Rust toolchain, an updated dependency or a refactor can slow a hot path without anyone meaning to. Our plan is to keep the benchmark running over time, add the new paths as they land, and record each change in the figures, so that additions and removals can be checked against the numbers that came before them.

Linux and Windows coverage is the other large piece. Both platforms matter to people who run fiskus, and we want figures for each. Getting them right takes more work than the macOS numbers, because the machines, the memory measurements and the tooling all differ, so we will add them carefully and will not rush the figures.
