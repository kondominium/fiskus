# Roadmap

fiskus is three tools built on one Rust core:

1. **Core** (`src/`, published crate): the tax computation.
2. **Docs** (`docs/`, mdBook): the guide, formulas, and decisions.
3. **Calculator** (`docs/calculator/`): a browser app that runs the core as WebAssembly behind a React front end.

Each tool ships on its own schedule. This page is the single place that says where each one is.

## Status

| Tool | Today | Next |
|---|---|---|
| Core | Tariff of § 32a EStG for 2006–2026, basic allowance, `explain` | Income computation (§ 2 EStG): Einkünfte, Werbungskosten, Sonderausgaben, Vorsorgeaufwendungen, außergewöhnliche Belastungen, Progressionsvorbehalt, Entlastungsbetrag, Altersentlastungsbetrag, children (Kinderfreibetrag, BEA, Kindergeld Günstigerprüfung), Anlage V, § 10d |
| Docs | Guide, formulas, KaTeX, charts, decisions | Golden fixtures, packaging notes, deployment |
| Calculator | Tariff calculator only (taxable income in, tax out) | Income inputs once the core has them |

## Milestones

- **v0.1.0** — tariff reference engine for 2026, exact arithmetic, provenance on every number, browser calculator as a tariff calculator. Needs Steuerberater sign-off on `rules/2026/parameters.toml`.
- **v0.2.0** — income computation, so the calculator can answer what most people ask. Includes Kinderfreibetrag, BEA and Kindergeld Günstigerprüfung, Sonderausgaben, Vorsorgeaufwendungen, Werbungskosten, außergewöhnliche Belastungen and Progressionsvorbehalt.
- **v0.3.0** — golden oracle comparison green, packaging and release automation, public mdBook.
- **v0.4.0** — Solidaritätszuschlag (Soli) and Kirchensteuer.
- **v0.5.0** — further income types and deductions: Kapitalvermögen with Abgeltungsteuer (§ 32d), Gewerbebetrieb, Selbständige Arbeit, foreign income credit (§ 34c), Haushaltsnahe Dienstleistungen (§ 35a), Entfernungspauschale.
- **v1.0.0** — years 2020–2025, sign-off complete.

Deferred: 2027 projection, other taxes (`fiskus-grundsteuer`, `fiskus-grunderwerbsteuer`).

## Notes

- Detailed checkpoints live in `.agents/plans/2026-11-21-fiskus-handoff.md`.
- The gap review (what the plan promised vs. what exists) lives in `.agents/research/plan-gap-review.md`.
- Adopted decisions are recorded in `docs/decisions/`. Options under discussion stay in `.agents/research/`.