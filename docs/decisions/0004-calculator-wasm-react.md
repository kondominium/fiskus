# 0004: Calculator in the browser: Rust core in WebAssembly, React front end

Status: accepted for the tariff calculator (v0.1). Revisit when income computation (§ 2 EStG) lands.
Date: 2026-11-22

## Context

fiskus should offer an interactive calculator, like the Federal Ministry of Finance's (BMF) online tool, that takes an income and shows the tax, the zone, the formula with the numbers, the tariff curve and the sources. The core has no runtime dependencies and reads no files, so it can run in a browser without changes. The BMF calculator could not be inspected during this work, because its site returned HTTP 503, so the interface is designed from the features the task asks for, not copied from it.

## Decision

1. **The core runs in the browser as WebAssembly.** The crate `docs/calculator/wasm` (not published) wraps the core in a small JSON interface. It exports `fiskus_alloc`, `fiskus_free` and `fiskus_call`, with no `wasm-bindgen`, so no second code generator is needed. The module is 119 KB before compression.
2. **Explanations come from the same code as the tax.** `explain` and `tariff_zones` in the core share the zone evaluation with `assess`, so the formula shown and the tax computed cannot disagree. Each explanation carries the statute citation, the source and the archived copy (provenance for the tariff; see the planning notes).
3. **React 19, TypeScript and Vite 8** for the front end, in `docs/calculator/app`. Tests run in Vitest against the same wasm module, and check the core's answers and the number formats.
4. **Design tokens** come from `docs/contributing/design.md`: fir green and paper white, the logo in its light and dark variants, and the signal red used once, for the marker. Fira Sans is requested, with a system fallback, since the font is not shipped.
5. **Static hosting.** The build uses a relative base, so it runs from any folder, for example GitHub Pages.
6. **Tools are pinned.** `mise.toml` pins Node 26 and adds the `wasm32-unknown-unknown` target to the Rust toolchain. `make web` runs the tests and the build, and CI has a `web` job.

## Consequences

- The calculator shows the tariff of § 32a EStG for every year from 2006 to 2026, with the basic allowance and all zones. It does not compute income, allowances, the solidarity surcharge (Solidaritätszuschlag) or church tax (Kirchensteuer), and the page says so.
- The repository gains an npm dependency tree, pinned by `package-lock.json`. Its supply chain needs the same review as the Rust dependencies, which has not been set up yet.
- Dark mode follows the operating system, but it was not checked visually, because the headless browser used here stayed in light mode.

## Open

- Income computation (§ 2 EStG), which the calculator needs before it can answer what most people ask.
- Interface language. The labels are English, with German terms in parentheses, as the language rules require for docs. A German interface is a separate decision.
- Browser tests (for example Playwright) and an accessibility audit.
- Size: `serde_json` can be replaced by a hand-written encoder to shrink the module.
