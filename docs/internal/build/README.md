# How fiskus is built

A tax library has to behave the same way on every machine, in every year it covers. fiskus therefore does not read its tax rules from disk while it runs. The rules are checked and compiled into the library at build time, so the finished binary carries everything it needs and nothing it could lose, corrupt or misread.

For how the library is tested, see [How fiskus is tested](../testing/README.md).

{{#include ../assets/illustrations/build-flow.svg}}

The first three steps run at build time. The last step is the binary at runtime. It holds every year and reads no files.

## The steps

**Source files.** Each tax year has one file of parameters at `rules/<year>.toml`. Each parameter has its value, its citation, its source, the date it was retrieved, and a `reference` link to the archived source. Changing a year means changing one file.

**Build check.** While the library builds, every file is validated. The build stops, with a message that names the file, for a value that is negative, a blank citation, source, retrieval date or reference, or a zone table that does not fit: a first zone that does not start at the allowance plus one euro, zones out of order, a step that is not positive, a tax that jumps by more than one euro at a zone start, or zones without a `[source]`. Bad data does not reach a release.

**Generated code.** The checked values are written out as Rust code: one basic allowance, one tariff table and one source per tax year. `for_year`, `tariff_for`, `tariff_source_for` and `solidarity_for` look them up by `TaxYear`. A year without a tariff has no zones and no source. A year without a solidarity threshold has none. Nothing about the tables is decided at runtime.

**The binary.** The compiler places those static values in the binary. At runtime, looking up a year's value is a read of a constant in memory. There is no file to open, no path to misconfigure, and no file that could be edited or corrupted after the build.

## Why this way

| Alternative | Why fiskus does not use it |
|---|---|
| Read the parameter files at runtime | Adds a runtime dependency and an error path for data that never changes after the build. Files on disk can be missing, edited or corrupted. |
| Write the values as plain constants by hand | Loses the citations and sources, which keep each value reviewable as data. |
| A procedural macro in its own crate | Needs a second crate to version and publish, for a few years of values. |

The build check keeps the values in files that are easy to review, fails the build on bad data, and leaves the library with no runtime dependencies.

## Size and dependencies

- The binary gains the values, not the files they came from. The cost is a few numbers per year.
- The library has no runtime dependencies. The parsing tools run only at build time.
- The tariff's citation, source and archived copy are in the binary, and `explain` returns them. The basic allowance's citation, and every retrieval date, stay in the files for review. Carrying every provenance field in the binary is planned, and it is not yet in fiskus.

## Proofs and fuzzing

The proofs take parameter values as arguments, or as symbolic zones with stated bounds. No file is read inside a proof, so the proofs check the same code that ships. The fuzz targets in `fuzz/` link the library as a dependency and call its public API, so they check the same build.

## Packaging

`Cargo.toml` includes `build.rs`, `rules/**/*.toml` and the source. `docs/references` is not packaged, and the build does not read it: each `reference` is a link to the GitHub main branch.
