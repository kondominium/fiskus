{{#include ../assets/icons/build.svg}}

# How the build fits together

Think of the build as the kitchen behind fiskus. It takes the tax data, checks it, and puts it inside the library. The library never opens a file while it runs.

This page describes how that is arranged and why. For the API, see [Using fiskus](../../usage/). For how the results are checked, see [How we test](../test/).

## Data becomes code

The tax figures are plain data, one file per year in `rules/`. Each figure carries its citation, its source and the date it was retrieved, so the file reads like a reference. The build turns those figures into constants the compiler knows about.

{{#include ../assets/illustrations/build-bundle.svg}}

The flow has four stages. `build.rs` reads the year's file and checks each entry. It writes the values into Cargo's `OUT_DIR`, a folder for build output that is never committed. `src/params.rs` brings those values in, and `for_year` picks the set that matches a `TaxYear`. Nothing is read at runtime, so the binary is self-contained.

Only the value reaches the code. The citation, source and date stay in `rules/` as reference, because the code does not track provenance yet (see the [code rules](../../contributing/code.md)).

The build is also the first line of defence. A value that is negative, not a whole euro amount, or missing its citation stops the build, with a message that names the file and the key. Bad data never reaches a test.

## Why `build.rs`

Parameters are fixed when the crate is built, so that is where they are checked. Three other shapes were considered and set aside:

- Parsing TOML at runtime adds runtime dependencies and an error path for data that never changes after the build.
- Plain Rust constants lose the citations and sources, which keep each value reviewable as data.
- A procedural macro in its own crate needs a second crate to version and publish, which is not worth it for one year of parameters.

A build script keeps the values in TOML, fails the build on bad data, and keeps the library free of dependencies.

## Parameters and years

A parameter lives in two places: its value in the year's TOML file, and a field in `Parameters` that reads it. The compiler joins the two. A field without data does not compile. Data without a reader produces a dead-code warning, and `make check` treats warnings as failures. So a parameter cannot go unused, or be read before it exists in the data.

A tax year works the same way. Each year has its own file, and each `TaxYear` variant has a match arm in `for_year`. The compiler requires every variant to have one, so a year cannot be half-added.

## Commands and tools

Day-to-day work runs through `make`. A target is a name for a job: `mise` holds the exact command, and cargo does the work. The names stay stable even when the commands change, and the `##` comments in the Makefile list what exists.

{{#include ../assets/illustrations/build-commands.svg}}

The picture shows the main idea. One entry point reaches the library, its examples and tests, the guide's charts, the book, and the proofs. Each output comes from the same tool chain, so there is one place to look when something breaks.

## Parts beside the library

Cargo looks for examples in `examples/` and tests in `tests/` by default. Tools that are not part of the library live in `scripts/` and are registered in `Cargo.toml`. The guide's chart generator and its checks sit in `scripts/guide/`, next to the guide they serve, and out of the library's public examples.

## What ships in the crate

The `include` list in `Cargo.toml` decides what goes into the published package. The package has to contain the build script and the rules, because the build needs both. The guide, the scripts and these internals pages stay out of it.

Dependencies follow the same thinking. `serde` and `toml` are build dependencies, so the library itself has none.

## Kani and the build

The build script runs for the proofs too. The proofs receive the parameter values as arguments, so no TOML is parsed inside a proof. See [How we test](../test/#proofs).
