{{#include assets/icons/calculator.svg}}
# Calculator

The calculator is a React application that runs in the browser. It does not reimplement any tax logic: every number comes from the Rust core, compiled to WebAssembly.

## Where it lives

- `docs/calculator/wasm` builds the core into `fiskus_web.wasm`. It is a `cdylib` with no `wasm-bindgen`; the host talks to it through three C functions.
- `docs/calculator/app` is the React 19 / TypeScript / Vite 8 front end. It loads `fiskus.wasm` and calls the core through a typed client in `src/lib/fiskus.ts`.

## How the front end uses the Rust core

The wasm module exports `fiskus_alloc`, `fiskus_free` and `fiskus_call`. The TypeScript client writes a JSON request into module memory, calls `fiskus_call`, reads the JSON answer, and frees both buffers. The request carries an `op` field; the supported operations are:

- `meta`: list the tax years and filing statuses.
- `explain`: compute the full assessment and explanation for one income.
- `zones`: return the tariff zones and their statute citation.
- `curve`: sample the tax for a range of incomes.
- `rates`: sample the average and marginal rates.
- `compare`: compare the tax across all years for one income.

The core exports `assess`, `explain`, `tariff_zones` and `basic_allowance_eur`, so the formula shown and the tax computed come from the same code. Each explanation carries the statute citation, the source and the archived copy, which the page shows verbatim.

## Building

`make web` runs the tests and the build. `make web-wasm` builds the wasm module and copies it, with the logos, into `docs/calculator/app/public`. The Vite build uses a relative base, so the result runs from any folder, including GitHub Pages.

## Publishing

`make pages` builds the book and the calculator, then copies the calculator build into `target/book/calculator`. The resulting `target/book` directory is what gets deployed to GitHub Pages.