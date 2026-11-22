# Build

This page describes how the tax parameters in `rules/` become part of the compiled crate, and how `make` and `mise` run the build. It is for people changing the build. For the API, see [Using fiskus](../../usage/README.md).

## Files

| File | Role |
|---|---|
| `rules/<year>/parameters.toml` | One table per parameter, with its value and sources. The data. |
| `build.rs` | Reads, validates and converts the TOML into Rust code at build time. |
| `src/params.rs` | Declares the types the generated code fills in, and maps each `TaxYear` to its values. |
| `src/year.rs` | The `TaxYear` enum. |
| `Cargo.toml` | Dependencies, and the `include` list that decides what ships in the `.crate`. |
| `Makefile`, `mise.toml` | The entry points for developers (`make build`, `make check`, and so on). |

## How the parameters reach the binary

1. `build.rs` lists the years it reads in its `YEARS` constant. For each year it reads `rules/<year>/parameters.toml`.
2. It checks every entry (see [Validation](#validation)).
3. It writes `parameters.rs` into Cargo's `OUT_DIR`. For 2026 this is:

   ```rust
   static PARAMETERS_2026: Parameters = Parameters {
       basic_allowance: Parameter { value_eur: 12348 },
   };
   ```

4. `src/params.rs` includes that file with `include!`, and `for_year` returns the static for the requested year.

Nothing is read from the filesystem at runtime. The values are compiled into the binary.

Only `value_eur` reaches the code. The citation, source and retrieval date stay in `rules/` as reference, because provenance is not tracked by the code yet (see the [code rules](../../contributing/code.md)).

`build.rs` re-runs when a year's `parameters.toml` changes.

## Validation

The build fails if an entry breaks one of these rules:

- The file is not valid TOML, or an entry has a field other than `value_eur`, `citation`, `source` or `retrieved`.
- `value_eur` is not a whole number. Decimal euro amounts are rejected.
- `value_eur` is negative.
- `citation`, `source` or `retrieved` is blank.
- A key is not lowercase snake_case. Keys become Rust field names.

Each message names the file and the key, for example `rules/2026/parameters.toml: [basic_allowance] citation is empty`. TOML syntax errors include the line and column.

Some failures happen at compile time rather than in `build.rs`. Those are covered in [Adding a parameter](#adding-a-parameter) and [Adding a tax year](#adding-a-tax-year).

## Adding a parameter

1. Add a table to `rules/<year>/parameters.toml` with `value_eur`, `citation`, `source` and `retrieved`.
2. Add a field of type `Parameter` with the same name to `Parameters` in `src/params.rs`.
3. Read it from the code, for example through `for_year(year).<name>.value_eur`.

Step 2 is required. Without it, the generated code does not compile, with `struct Parameters has no field named ...`. If the field is added but never read, rustc warns that it is never read, and `make check` fails on that warning.

## Adding a tax year

1. Create `rules/<year>/parameters.toml`.
2. Add the year to `YEARS` in `build.rs`. If the file is missing, the build fails with `No such file or directory`.
3. Add a variant to `TaxYear` in `src/year.rs`.
4. Add a match arm in `for_year` in `src/params.rs`.

Step 3 and step 4 are checked by the compiler. A `TaxYear` variant without a match arm is a non-exhaustive match error.

Document the year on the [usage page](../../usage/README.md#tax-year).

## Dependencies

- `serde` and `toml` are build dependencies. The library has no runtime dependencies.
- They are also dev dependencies, for tests that read the TOML files.

Add build-only crates under `[build-dependencies]`, so they stay out of the library.

## Packaging

The `include` list in `Cargo.toml` decides what goes into the `.crate`. It must contain both:

- `/build.rs`, because the build script is needed to compile the crate.
- `/rules/**/*.toml`, because `build.rs` reads these files.

Check the list with `cargo package --list`.

## Kani

The build script runs for Kani proofs as well. The proofs pass the parameter values into the function they check, so no TOML is parsed inside a proof (see the [code rules](../../contributing/code.md)).

## Make and mise

- Each `make` target calls a task in `mise.toml`. The task holds the cargo command, so the two stay in sync.
- `make help` builds its list from the `##` comments in the Makefile, and sizes the column to the longest target name.
- The Makefile is silent (`.SILENT:`), so only the output of the commands is shown.
- `make build` runs `cargo build --all-targets`, which also compiles the examples and tests.
- `make run-example-<name>` runs `cargo run --example <name>`. Each example needs one target in the Makefile and one task in `mise.toml`.
- `make check` runs formatting, clippy with warnings denied, the tests and the doc tests.
- `make verify` runs the Kani proofs.

## Why `build.rs`

The parameters are fixed when the crate is built, so the build is where they are checked. The alternatives were rejected for these reasons:

- **Parse TOML at runtime.** Adds runtime dependencies and a runtime error path for data that does not change after the build.
- **Plain Rust constants.** Loses the structured citations and sources, which keep each value reviewable as data.
- **Procedural macro in a separate crate.** Needs a second crate to version and publish. That is not worth it for one year of parameters.

A build script keeps the values in TOML, fails the build on bad data, and keeps the library free of dependencies.
