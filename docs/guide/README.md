{{#include assets/icons/overview.svg}}

# How your taxes are calculated

Income tax (Einkommensteuer) is charged on the income a person earns in a year. It is assessed for a filing unit (Veranlagungseinheit): one person, or two spouses assessed together.

The tax office (Finanzamt) makes the assessment. The rules are in the [Income Tax Act (Einkommensteuergesetz, EStG)](https://www.gesetze-im-internet.de/estg/), which the Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) administers.

The calculation runs in a fixed order, and this guide follows it. Each page explains one step: first in plain words, then the rule and the fiskus type that implements it. This page only maps the steps. Steps that fiskus does not implement yet are marked, and they are not explained until they are.

| Step | What it is | In fiskus |
|---|---|---|
| 1. Who is taxed together | Filing unit and filing status | Implemented: [Who is taxed together](01-who-is-taxed-together.md) |
| 2. Tax year | Assessment period (Veranlagungszeitraum) | Implemented: [The tax year](02-tax-year.md) |
| 3. Income types | Income from the seven types (Einkünfte) | Not yet in fiskus |
| 4. Allowances and special expenses | Amounts that reduce income before taxable income | Not yet in fiskus |
| 5. Taxable income | The amount the tariff applies to (zu versteuerndes Einkommen, zvE) | Input today: [Taxable income](03-taxable-income.md) |
| 6. Tariff tax | Tax on taxable income (tarifliche Einkommensteuer) | First zone only: [The basic allowance](04-basic-allowance.md) |

## How the pieces fit

Income tax is worked out in layers. Each layer starts from what is left over from the one before.

$$
\begin{aligned}
&\fcolorbox{#0f3d2e}{#f3f0e8}{\textcolor{#0f3d2e}{\text{Taxable income}}} = \colorbox{#0b2545}{\textcolor{#f3f0e8}{\text{Total income}}} - \colorbox{#0b2545}{\textcolor{#f3f0e8}{\text{Allowances}}} \\
&\qquad - \colorbox{#0b2545}{\textcolor{#f3f0e8}{\text{Special expenses}}} - \colorbox{#0b2545}{\textcolor{#f3f0e8}{\text{Child allowances}}} \\
&\qquad\qquad \rightarrow \colorbox{#e0391f}{\textcolor{#f3f0e8}{\text{Tariff tax}}}
\end{aligned}
$$

The outlined box is the input today. Blue boxes are not yet in fiskus. The red box is the result.

1. Total income. Your income from the seven income types (Einkünfte), such as work, investments, rent and business. *Not yet in fiskus.*
2. Minus allowances and special expenses. First some allowances, then special expenses (Sonderausgaben) and extraordinary burdens (außergewöhnliche Belastungen). *Not yet in fiskus.*
3. Minus child allowances (Freibeträge). The allowances for each child. What is left is taxable income (zu versteuerndes Einkommen, zvE). *Not yet in fiskus.*
4. Tariff tax on taxable income. The tariff is applied to taxable income. Its first part, the basic allowance (Grundfreibetrag), has zero tax. The basic allowance is not subtracted from income. It is a zone of the tariff.

**In short: taxable income is total income, minus allowances and special expenses, minus child allowances.** The tariff tax is calculated from taxable income.

Some benefits, such as parental allowance (Elterngeld), are not part of taxable income. Their amount is worked out under [their own rules](https://www.gesetze-im-internet.de/beeg/__2.html). They can still change your tax, through the progression reservation (Progressionsvorbehalt): the rate applied to your taxable income can go up when such benefits are paid. *Not yet in fiskus.*

Today fiskus takes taxable income as an input. Layers 1 to 3 are not yet in fiskus.

fiskus is an independent reference engine. It is not official BMF or ELSTER software, and it is not tax advice.
