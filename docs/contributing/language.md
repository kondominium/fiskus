# Language

Applies to humans and agents who write docs, the guide, and doc comments. Code identifiers follow the [code rules](code.md).

## Docs and doc comments

- Write prose in English. Statute citations and quoted statute text stay German.
  - Good: `The basic allowance rises each year.`
  - Bad: `Der Grundfreibetrag steigt jedes Jahr.`
  - Why: readers and search work in one language; only the statute's own words stay German.

- **Term format:** the English term comes first, and the German term follows in parentheses, on first use per page. Example: tax office (Finanzamt). After that, use the English term only.
  - Good: `The tax office (Finanzamt) assesses the tax. Later the tax office checks the return.`
  - Bad: `The Finanzamt (tax office) assesses the tax.`
  - Why: the English term leads, so the page reads in English; the German alias is a lookup aid used once.

- **Statute names:** the English name, then the German name in parentheses, then the abbreviation. Example: Income Tax Act (Einkommensteuergesetz, EStG).
  - Good: `Income Tax Act (Einkommensteuergesetz, EStG)`
  - Bad: `EStG (Income Tax Act)`
  - Why: the English name leads, and the abbreviation is the handle readers search for.

- **Glossary:** every term used in the guide is registered in the [glossary](../guide/glossary.md), under the first letter of its English name. Add a term there before using it on a page.
  - Good: add `taxable income (zu versteuerndes Einkommen, zvE)` to the glossary, then use it.
  - Bad: use `zvE` on a page without a glossary entry.
  - Why: the glossary is the single place a reader checks a term; an unregistered term drifts.

- Write `fiskus` in lowercase, including at the start of a sentence.
  - Good: `fiskus computes the tariff.`
  - Bad: `Fiskus computes the tariff.`
  - Why: the name is a common noun in this project, and consistency makes search reliable.

- Cite one section per citation, in the form `§ 32a Abs. 1 Satz 2 Nr. 1 EStG`. Do not use `§§` lists. The guide tests check this.
  - Good: `§ 32a Abs. 1 Satz 2 Nr. 1 EStG`
  - Bad: `§§ 32a, 32b EStG`
  - Why: one section per citation keeps each claim traceable to one place in the statute.

- Write euro amounts as `12 348 €`: a space between thousands, a point for cents (`12 348.99 €`), and the euro sign after a space.
  - Good: `12 348 €` and `12 348.99 €`
  - Bad: `12,348€` or `€12348`
  - Why: one unambiguous format keeps amounts scannable and avoids locale confusion.

- Say what a rule does, not what the reader should do. The guide gives no tax advice.
  - Good: `Joint filing halves the taxable income before the tariff.`
  - Bad: `You should file jointly to reduce your tax.`
  - Why: the guide describes the statute; it must not advise a filing choice.

- Name the authority by its role first: the tax office (Finanzamt), the Federal Ministry of Finance (BMF).
  - Good: `the Federal Ministry of Finance (BMF)`
  - Bad: `the BMF (Federal Ministry of Finance)`
  - Why: the role tells the reader who is meant before the abbreviation.

- Say what fiskus implements. Anything it does not implement is marked "not yet in fiskus".
  - Good: `Solidaritätszuschlag is not yet in fiskus.`
  - Bad: `fiskus will soon support Solidaritätszuschlag.`
  - Why: stating status plainly prevents readers from assuming a feature exists.

- Source files start with the English and German disclaimer lines, as in `src/money.rs`.
  - Good: the two disclaimer comment lines at the top of the file.
  - Bad: a file with no disclaimer.
  - Why: every source carries the not-official, not-tax-advice notice.

- The guide's overview page carries the disclaimer: not official BMF or ELSTER software, and not tax advice.
  - Good: the disclaimer on the overview page.
  - Bad: the disclaimer only in a footnote.
  - Why: the notice must be visible where the guide begins.
