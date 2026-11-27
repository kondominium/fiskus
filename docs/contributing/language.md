# Language

Applies to humans and agents who write docs, the guide, and doc comments. Code identifiers follow the [code rules](code.md).

## Docs and doc comments

The guide tests (run by `make test-all`) check the rules a program can check, on every prose page in `docs/`, the agent skills and the root README: no `we`, `our` or `us`, no `§§`, euro amounts in the form `12 348 €`, and `fiskus` in lowercase. Code, inline code, URLs, HTML comments and text in double quotes are not checked. Quotes are how a page shows wrong usage, such as "We test every figure."

- Write prose in English. Statute citations and quoted statute text stay German.
- **Term format:** the English term comes first, and the German term follows in parentheses, on first use per page. Example: tax office (Finanzamt). After that, use the English term only.
- **Statute names:** the English name, then the German name in parentheses, then the abbreviation. Example: Income Tax Act (Einkommensteuergesetz, EStG).
- **Glossary:** every term used in the guide is registered in the [glossary](../guides/glossary.md), under the first letter of its English name. Add a term there before using it on a page.
- Write `fiskus` in lowercase, including at the start of a sentence.
- Do not write "we", "our" or "us". Make `fiskus` the subject. Bad: "We test every figure." Good: "fiskus tests every figure." Why: the docs describe the project, and a project reads as more professional than a team talking about itself.
- Cite one section per citation, in the form `§ 32a Abs. 1 Satz 2 Nr. 1 EStG`. Do not use `§§` lists. The guide tests check this.
- Write euro amounts as `12 348 €`: a space between thousands, a point for cents (`12 348.99 €`), and the euro sign after a space.
- Say what a rule does, not what the reader should do. The guide gives no tax advice.
- Name the authority by its role first: the tax office (Finanzamt), the Federal Ministry of Finance (BMF).
- Say what fiskus implements. Anything it does not implement is marked "not yet in fiskus".
- Never refer to planning notes in the docs: no slice, phase or checkpoint numbers, and no status of planned work. Planning status goes stale. Say what the code does, in plain words.
- Source files start with the English and German disclaimer lines, as in `src/money.rs`.
- The guide's overview page carries the disclaimer: not official BMF or ELSTER software, and not tax advice.

## Voice

The guide explains German income tax as a friendly explainer. Use short sentences and one idea per paragraph. Open each page with the everyday question it answers, and show one concrete example before the rule. Say what a rule does, and give no tax advice.

A claim about fiskus carries its evidence: a generated number, a chart, or a link to the test that enforces it. A superlative needs a figure, so write "about 42 million assessments per second on one core", not "very fast".

Use precise words for authority. Write "checked against the BMF calculator". Do not describe fiskus as "certified", "official" or "approved by" the BMF. In `docs/`, do not write "guaranteed", "100 % accurate", "best" or "perfect".
