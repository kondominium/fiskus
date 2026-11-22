# 0001: Sources and references

Status: accepted for the basic allowance (Grundfreibetrag). Open items are listed below.
Date: 2026-11-22

## Context

Every parameter needs a source the reader can check. The Lohnsteuer repository (https://github.com/MarcelLehmann/Lohnsteuer, Apache-2.0) generates a withholding calculator from the Federal Ministry of Finance's (BMF) Programmablaufplan (PAP) for 2006 to 2026. It links the PAP PDFs. We checked its links and the PAP PDFs themselves. We did not run or review its generated code.

## Decision

1. **Source tiers.** A **primary** source is the official Bundesgesetzblatt (BGBl.) text. An **official** source is a BMF or Bundestag document that states the amount. The PAP is official guidance that implements the statute, and its value is confirmed, but the statute is not quoted. An **unquoted secondary** source is a third-party transcription. It is kept as a record and superseded where an official source covers the same value.
2. **The Lohnsteuer repository is a lead source.** We use its list of PAP URLs and cite the BMF PDFs directly. We copy no code from it, and its output is not a source.
3. **The PAP confirms values, it is not the assessment oracle.** Withholding (Lohnsteuer) and assessment (Einkommensteuer) differ in conventions. The PAP confirms the basic allowance, which is the same number in both. Version 0.1 covers Einkommensteuer only.
4. **Each parameter names its sources.** The `citation` field gives the statute and the setting law. The `reference` field points to the shadow copy in `docs/references/`. Each file there has a sha256 in its README.

## Consequences

- The basic allowance history covers VZ 2006 to 2026. VZ 2002 to 2005 are not yet in fiskus.
- These changes depart from the handoff plan (`.agents/plans/2026-11-21-fiskus-handoff.md`), and are recorded here:
  - Sources live in `docs/references/` with a README, not in `sources/` with `SHA256SUMS` (plan §2 Sources, §5 Checkpoint 0).
  - Parameters are per year, `rules/<year>/parameters.toml`, and the history from 2006 to 2025 is in `rules/`. The plan had 2026 only and no 2020 to 2025 on day one (plan §2 Bundling, §6 traps). The owner asked for the history.

## Open

- **Redistribution.** Plan §3.6 asks whether BMF and Bundestag documents may be committed. The BMF imprint page returned HTTP 503, so this is unverified. Until it is checked, the PDFs should not be committed. One option is to commit only the hashes and keep the PDFs in a git-ignored cache.
- **Packaging.** `build.rs` fails if a `reference` file is missing. The crate's `include` list leaves out `docs/references`, so a crate built from the published package would not compile. This must be fixed before publishing.
- **BMF pseudocode (plan §3.1).** https://www.bmf-steuerrechner.de/interface/pseudocodes.xhtml returned HTTP 503 again during this check.
- **PAP XML (plan §3.2).** The BMF publishes the PAPs as PDF. The repository links only PDFs, and no XML was found. The bmf-steuerrechner site is unreachable, so the check is incomplete.
- **Provenance.** Plan §6 requires every number in `Assessment` to carry its § citation and parameter version. `docs/contributing/code.md` says provenance is a later feature. The two need one decision.
- **ADRs from the plan.** License, repository topology, naming, verification strategy and non-affiliation wording are not yet written. This is the first ADR in `docs/decisions/`.
