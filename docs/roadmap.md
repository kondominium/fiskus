# Roadmap

This page lists the features and domains fiskus plans to cover, in planned order. Everything here is not yet in fiskus, except the basic allowance (Grundfreibetrag) for tax year 2026, which fiskus implements today.

The order is deliberate. Each milestone depends on the ones before it. The build and the test process are proved on simple values before the complex formulas arrive.

## 1. Basic allowance for every tax year

**Goal:** the build and the test process work for every tax year, using only the basic allowance.

- Add one `rules/<year>/parameters.toml` for each tax year, with the value, citation, source and retrieval date.
- Run the build check and the year tests for every year.
- Check that the packaged crate builds with every year file in it.
- Follow the steps in [Tax parameters](contributing/code.md#tax-parameters) for each new year.
- Check the sources regularly for changes to each year's values, and retrieve the new source when a value changes.

## 2. Kani proof fixes

**Goal:** the proofs check what they claim, before the tariff work depends on them.

- Prove the joint boundary against an independent statement: a joint filer is in the zone if and only if the whole euros are at most twice the allowance plus one.
- Prove `whole_euros` for every `i128` value: it is the floor of the cents divided by 100.
- Use one bound name for euro values and another for cents.
- State in each proof's comment what it covers and what it does not, such as the allowance being a free input.

## 3. Rounding

**Goal:** every rounding step is stated, tested and documented in the order the statute applies it.

- Taxable income is rounded down to the whole euro (§ 32a Abs. 1 Satz 1 EStG).
- The formula result is rounded down to the whole euro (§ 32a Abs. 1 Satz 6 EStG).
- For joint filers, the half of the income is rounded down, taxed, and the result is doubled (§ 32a Abs. 5 EStG).
- Write the rounding rules on a page of their own under `docs/formulas/`.
- Test each step with the values on either side of it.

## 4. Income tax from taxable income to total tax

**Goal:** calculate the total tax from taxable income (zu versteuerndes Einkommen). The total tax is the income tax (Einkommensteuer), the solidarity surcharge (Solidaritätszuschlag) and church tax (Kirchensteuer).

- Tariff zones 2 to 5 of § 32a Abs. 1 EStG, for single filers.
- Joint filing with the splitting procedure (§ 32a Abs. 5 EStG). The joint tax is always twice the tax on half the income, and always a whole number of euros.
- Widowed filers, for the year after the spouse dies (§ 32a Abs. 6 EStG).
- The solidarity surcharge, under the Solidarity Surcharge Act (Solidaritätszuschlaggesetz 1995, SolZG).
- Church tax, which each state sets for its members.
- Kani proofs for the formulas over bounded inputs: no panic, and exact zone boundaries.
- Check that adjacent formulas agree at each threshold, to within one euro.
- Fuzz testing of the calculation path. Not yet in fiskus. Update the testing page when it lands.

## 5. Conformance with the BMF calculator

**Goal:** find bugs by comparing fiskus with the results of the Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) calculator.

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

- The average tax rate, also called the effective tax rate (Durchschnittssteuersatz), as a percentage of taxable income.
- The marginal tax rate (Grenzsteuersatz), as the tax on the next euro, as a percentage.
- The tariff zone (Tarifzone) that a figure falls in, with the statute section that sets it.
- An explanation for each figure, with a reference to the statute section behind it.
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
