# Roadmap

This page lists the features and domains fiskus plans to cover, in planned order. Each milestone says what is done, and what is still to do. Everything not marked done is not yet in fiskus.

The order is deliberate. Each milestone depends on the ones before it. The build and the test process are proved on simple values before the complex formulas arrive.

## 1. Basic allowance for every tax year

**Done:** each listed year has a `rules/<year>.toml` with its basic allowance, citation, source and retrieval date. The build checks every file. The listed years are in `TaxYear::ALL`.

**Still to do:**

- Check the sources regularly for changes to each year's values, and retrieve the new source when a value changes.

## 2. Kani proof fixes

**Done:** the proofs cover the zero zone for any zone table, the joint doubling and halving, the rejection of negative income, the solidarity surcharge bounds, and the saturation of `Cents::from_euros`. Each proof states its bound in a comment. The floor of a fraction is a property test instead, because the solver cannot finish a proof over 128-bit division in reasonable time.

**Still to do:**

- Prove the joint boundary against an independent statement: a joint filer is in the zone if and only if the half of the income is at most the allowance.
- Prove `whole_euros` for every `i128` value, not only a bound.

## 3. Rounding

**Done:** taxable income is rounded down to the whole euro (§ 32a Abs. 1 Satz 1 EStG), the tax is rounded down once at the end of the zone (§ 32a Abs. 1 Satz 6 EStG), and joint filers are taxed on the rounded-down half (§ 32a Abs. 5 EStG). The rules are on the [tariff](formulas/tariff.md) page, and the unit and property tests run them for every listed year.

## 4. Income tax from taxable income to total tax

**Done:** the tariff of § 32a Abs. 1 EStG for every listed year that has zones, for single and for joint filers. Average (effective) and marginal tax rates. Zone explanations through `explain`. The solidarity surcharge (§ 4 SolZG) for the years whose free threshold (§ 3 Abs. 3 SolZG) is in `rules/`: VZ 2024 to 2026 so far.

**Still to do:**

- Sources for the zones of years that have none yet, and the draft years checked against their enacted text.
- Widowed filers, for the year after the spouse dies (§ 32a Abs. 6 EStG).
- The solidarity surcharge threshold for the years before VZ 2024. Their statute (the amendments before the Steuerfortentwicklungsgesetz) is not archived yet. The rates of § 4 SolZG are checked against the statute text in `docs/references`.
- Church tax, which each state sets for its members.
- Fuzz tests of the surcharge and tariff paths beyond the smoke run, on a scheduled job.

## 5. Conformance with the BMF calculator

**Goal:** find bugs by comparing fiskus with the results of the Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) calculator. This is the next milestone. It lives on its own branch, so the unit, property and proof layers stay free of fixed figures per year.

- Take the BMF results for chosen incomes, about 2,000 cases planned. Not yet in fiskus.
- Check the terms of use of the BMF results before any fixture is committed to the repository.
- Run the cases as conformance tests, and record each difference as a bug.

## 6. Release and TypeScript-compatible driver

**Goal:** the first release, and a driver that TypeScript and JavaScript frontend code can use.

- Set the version policy and the changelog before the first release. The public API is pre-1.0.
- Build a WebAssembly (WASM) reference of the calculation. Not yet in fiskus.
- Publish a JavaScript / TypeScript package that wraps the WASM build, for frontend use. Not yet in fiskus.
- Run the public API tests against the WASM build, so both builds give the same results.

## 7. Derived figures and explanations

**Goal:** figures derived from the tax, each with the references that explain it.

**Done:** the average tax rate (Durchschnittssteuersatz) and the marginal tax rate (Grenzsteuersatz) in `explain`, the tariff zone (Tarifzone) that a figure falls in, and the citation and archived source of the year's tariff.

**Still to do:**

- An explanation for each figure that names its statute section, not only the year's source.
- Formulas shared by more than one zone or tax year are written once, and every place that uses them refers to that one place.

## 8. Taxable income from income (§ 2 EStG)

**Goal:** compute taxable income from income, instead of taking it as an input. Today fiskus takes taxable income as an input. This milestone is planned for later.

- The seven kinds of income (Einkunftsarten, § 2 Abs. 1 EStG).
- The total income (Gesamtbetrag der Einkünfte), after the offset of losses (§ 2 Abs. 3 EStG).
- The deductions that reduce total income to taxable income (§ 2 Abs. 4 EStG).
- Taxable income (§ 2 Abs. 5 EStG).

## 9. Online taxable income calculator

**Goal:** an online calculator that gives the same taxable income (zu versteuerndes Einkommen) as the online calculator of the Federal Ministry of Finance (Bundesministerium der Finanzen, BMF), in real time, with more data-rich values. Not yet in fiskus.

- Compute taxable income from the income inputs in real time, through the TypeScript-compatible driver (milestone 6).
- Match the BMF calculator for the same inputs, as checked by the conformance tests (milestone 5).
- Show the intermediate values behind each figure, with the statute section for each step, instead of only the final number.

## 10. Tariff exceptions

**Goal:** cover the cases where the basic tariff in § 32a Abs. 1 EStG does not apply to all of the income. Each exception changes how a part of the income is taxed.

- Progression clause for tax-free income (Progressionsvorbehalt), § 32b EStG.
- Flat tax on capital income (Abgeltungsteuer), § 32d EStG.
- Reduced rate for extraordinary income (außerordentliche Einkünfte), § 34 EStG.
- Reduced rate for retained business profits (nicht entnommene Gewinne), § 34a EStG.
- Reduced rate for income from timber felling (Holznutzungen), § 34b EStG.
- Credit for foreign income tax (ausländische Steuern), § 34c EStG.
- Each exception needs its own formula page, tests and guide update before it counts as implemented.

## 11. Language-neutral model and tests

**Goal:** under consideration, for later. Describe the tax model and its test cases in a form that any programming language can use, not only Rust.

- Write the model, the rules and the expected results as data, separate from the Rust code.
- Use the same test cases in every implementation, so a port is checked by the same tests as the Rust library.

## Future

A full tax assessment (Veranlagung) is unrealistic for now. It is large and complex. fiskus will still add features when they are needed.

The focus is clear, close-to-real numbers for the clients of the Kondominium app, which the library's creator builds. Those clients need figures they can run their own checks against. Each feature is added when it serves that need, with the same tests, statute references and "not yet in fiskus" markers as the rest of the docs.
