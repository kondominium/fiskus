# Language

Applies to humans and agents who write docs, the guide, and doc comments. Code identifiers follow the [code rules](code.md).

## Docs and doc comments

- Write prose in English. Statute citations and quoted statute text stay German.
- **Term format:** the English term comes first, and the German term follows in parentheses, on first use per page. Example: tax office (Finanzamt). After that, use the English term only.
- **Statute names:** the English name, then the German name in parentheses, then the abbreviation. Example: Income Tax Act (Einkommensteuergesetz, EStG).
- **Glossary:** every term used in the guide is registered in the [glossary](../guide/glossary.md), under the first letter of its English name. Add a term there before using it on a page.
- Write `fiskus` in lowercase, including at the start of a sentence.
- Cite one section per citation, in the form `§ 32a Abs. 1 Satz 2 Nr. 1 EStG`. Do not use `§§` lists. The guide tests check this.
- Write euro amounts as `12 348 €`: a space between thousands, a point for cents (`12 348.99 €`), and the euro sign after a space.
- Say what a rule does, not what the reader should do. The guide gives no tax advice.
- Name the authority by its role first: the tax office (Finanzamt), the Federal Ministry of Finance (BMF).
- Say what fiskus implements. Anything it does not implement is marked "not yet in fiskus".
- Source files start with the English and German disclaimer lines, as in `src/money.rs`.
- The guide's overview page carries the disclaimer: not official BMF or ELSTER software, and not tax advice.
