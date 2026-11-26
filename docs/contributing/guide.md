# Guide

The guide in `docs/guide/` explains how fiskus calculates income tax, one step at a time. It is live: it changes with the code, and a change is not complete until the guide matches it. The guide is complete when the code is.

## Scope

- The guide describes only what fiskus implements. Steps that are not implemented appear only in the map on the overview page, marked "not yet in fiskus".
- A page cites a statute section only if `docs/guide/coverage.toml` lists it for that page, and the code cites the section in `src/` or `rules/`.
- Figures on a page match the parameters in `rules/`. The guide tests check this.
- No placeholder pages. The roadmap lives in issues and code.

## Every change

Ask: does this change add or alter a rule, a parameter, a zone, an error, or how a filing unit is assessed? If yes, the same diff includes:

1. The updated page in `docs/guide/`, and its entry in `coverage.toml` if it cites a new section.
2. Regenerated charts, with `make charts`. Commit the SVG files.
3. `make test-all` passing. It runs the guide tests and the chart drift check.

If no, say why in the handover.

## Releases

A release is complete when `make gates` passes on it. The guide tests and the chart drift check then guarantee that the guide matches the code the release ships.

## Charts

- `scripts/guide/charts.rs` draws the charts from the public API (`assess`, `explain`, `tariff_zones`, `solidarity_surcharge`), so each chart shows what the code does. Charts use the latest listed year, and inputs scaled to its basic allowance, so they follow the rule files without edits.
- Each chart is one SVG in `docs/guide/assets/charts/`, with no background. Its ink and zone colors are CSS variables (`--chart-ink`, `--chart-zone`) set per theme in `docs/theme/fiskus.css`, so it follows the book's light and navy themes. Colors come from [design](design.md), with `signal` used once per chart as the highlight.
- Pages pull each chart in with `\{{#include assets/charts/<name>.svg}}`, so the SVG is inlined and picks up the theme. Do not link it with `<img>`: an image cannot read the page's variables.
- `make test-all` runs `cargo run --example guide_charts -- --check` and fails if a committed SVG, or the generated tariff table in `docs/formulas/tariff-coefficients.md`, differs from what the code and `rules/` produce.

## Illustrations

The guide has no illustrations. Every figure a chart shows is drawn from the code. A point that no code computes yet is written as text, with the words "not yet in fiskus", and not as a drawing.

## Icons

- Every guide page has one icon, placed directly above its H1 as `\{{#include assets/icons/<name>.svg}}`. `docs/theme/fiskus.css` floats it to the left of the title.
- Icons come from [Lucide](https://lucide.dev), fetched from `lucide-static`, not drawn by hand. Keep Lucide's license comment at the top of each file.
- Icons are 24-unit line icons. Each file sets `width` and `height` to 32, keeps `stroke="currentColor"`, and sets `style="color: var(--chart-ink, #0f3d2e)"` on the root `<svg>`, so the icon follows the theme.
- The Lucide notice lives in `docs/guide/assets/icons/LICENSE-LUCIDE.txt`. Keep it when icons are added or removed.
- `make test-all` fails if a page does not have exactly one icon, or if the icon file is missing.

## Page layout

Each page opens in plain words. It then states the rule with its statute section, adds a chart where one helps, and names the fiskus type or function that implements it.

## Agent instructions

Read this section before touching `docs/guide/`, `scripts/guide/charts.rs`, or `docs/theme/fiskus.css`. The [language rules](language.md) apply to all prose.

**Add a chapter**

1. Create `docs/guide/NN-slug.md` with the next number. Open with plain words, then the rule.
2. Cite statute sections as `§ 32a Abs. 1 Satz 2 Nr. 1 EStG`, one section per citation. Never `§§`.
3. Add a `[[chapter]]` entry to `docs/guide/coverage.toml`. List each cited article by number only (`"32a"`, `"26b"`). Only list sections the code cites in `src/` or `rules/`.
4. Add the page to `docs/SUMMARY.md` under `# Guide`, in order.
5. If a chapter quotes a parameter, write its citation verbatim and its value as `12 348 €`, as it appears in `rules/`.
6. Run `make test-all`.

**Add a chart**

1. Write a function in `scripts/guide/charts.rs` that takes no theme and returns `document(title, body)`. Use only the public API and `FilingUnit`, never internal modules.
2. Use the helpers (`text`, `line`, `polyline`, `circle`, `bar`) and the `INK`, `ZONE`, and `SIGNAL` constants. Do not write hex colors inline.
3. Register it in the `charts` array in `run()`.
4. Run `make charts`, then commit the SVG.
5. Embed it on the page with `\{{#include assets/charts/<name>.svg}}` on its own line, with a blank line before and after.
6. Run `make test-all`.

**Never**

- Edit an SVG by hand. Change the generator and regenerate.
- Use `<img>` or `<picture>` for a chart.
- Draw an illustration as a chart. Illustrations are the exception: see below.
- Use a dark-only or light-only color outside the tokens below.
- Draw a value or zone that `assess` does not return.

**Tokens**

Colors come from [design](design.md). Chart colors use CSS variables with light fallbacks.

| Token | Light | Navy (dark) | Used for |
|---|---|---|---|
| `--chart-ink` (`INK` in the generator) | `#0f3d2e` (fir) | `#f3f0e8` (paper) | Lines, text, bar outlines, filled bars |
| `--chart-zone` (`ZONE`) | `#f3f0e8` (paper) | `rgba(243, 240, 232, 0.18)` | Zone fill |
| `SIGNAL` | `#e0391f` (signal) | `#e0391f` (signal) | One highlight per chart |

Both variable sets live in `docs/theme/fiskus.css`: the `:root, .light` block and the `.navy` block. Add a new token to both, and to the generator's constants.

Font: Fira Sans, with `system-ui` and `sans-serif` as fallbacks (`FONT` in the generator).
