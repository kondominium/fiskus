{{#include assets/icons/overview.svg}}

# How income taxes are calculated in Germany

> **Disclaimer.** This is the fiskus team's reading of how income tax works. It is not tax advice, so use it at your own risk.

How much income tax does a person pay, and why does it differ from one neighbour to the next?

Two people can earn the same and still pay different tax. The answer comes from a fixed order of steps, and this guide walks through them in that order.

Income tax (Einkommensteuer) is charged on what a person earns in a year. It is not one flat percentage. The tax is worked out in steps.

{{#include assets/charts/rate-steps.svg}}

The solid line is the marginal rate, the tax on the next euro of taxable income. The dashed line is the average rate, the share of taxable income that goes to tax. Income is taxed in zones, and each zone taxes the next euro at a higher rate.

The rules are written in the Income Tax Act (Einkommensteuergesetz, EStG). The Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) administers that act, and the tax office (Finanzamt) makes the assessment (Veranlagung).

Let's look into how all of that is calculated, starting with the year the rules belong to.

## The tax year (Veranlagungszeitraum)

**The tax year is the calendar year, from 1 January to 31 December.** Only income received within those twelve months counts for that year.

{{#include assets/charts/calendar.svg}}

A calendar year ends on 31 December, and the next one starts the following day, 1 January. The chart shows 2025 and 2026 side by side, with the boundary between them marked.

Almost every figure changes from one year to the next, so every figure in this guide belongs to one tax year.

A tax year can still be claimed for a while after it ends. A person generally has four years to claim taxes back. A claim for 2022 stays open until 31 December 2026, and after that date it expires. A claim for 2021 has already closed.

<div class="info-box">

{{#include assets/callouts/calendar.svg}}

**Example Scenario:** In November 2026, Lena finds that she paid too much tax for 2022. She can still ask for it back until 31 December 2026. For 2021 that window has already closed.

</div>

<div class="info-box">

{{#include assets/callouts/lightbulb.svg}}

**How fiskus deals with that:** `assess` returns the tax under the rules of the `TaxYear` it is given, and `TaxYear::from_year` returns `None` for a calendar year that is not listed. fiskus lists the years from 2003 on, the earliest year for which it has rules ([Quick start](../usage/README.md#quick-start)).

</div>

With the year fixed, the next question is what the tax is charged on: the income that is left after the deductions.

## Taxable income (zu versteuerndes Einkommen, zvE)

**Taxable income is total income minus allowances.** The law takes several amounts off the income before the tax is worked out.

{{#include assets/charts/layers.svg}}

Total income (Gesamtbetrag der Einkünfte) adds up the income from the seven income types (Einkünfte), such as work, investments, rent and business. Allowances (Freibeträge) come next, then special expenses (Sonderausgaben), then child allowances. What is left is taxable income (zvE). The basic allowance is not one of these subtractions. It works inside the tariff, as the next section shows.

The deductions exist because the tariff taxes what a person can afford to pay. The law first takes out money a person must spend to live and to earn. Work-related expenses (Werbungskosten), such as the cost of travel to work, are the cost of earning income, so they come off. Extraordinary burdens (außergewöhnliche Belastungen), such as a long illness, cost more than ordinary living costs, so they come off too.

The state can also use the same kind of rule to reward choices it wants more of, such as energy-efficient renovation of a home or investment in renewable energy. Such rules sit in other parts of the Income Tax Act, and they lower either the income that is taxed or the tax due (Steuervergünstigung).

Once taxable income is known, the first question is how much of it is tax-free at all. That is the basic allowance.

<div class="info-box">

{{#include assets/callouts/lightbulb.svg}}

**How fiskus deals with that:** fiskus does not compute deductions today. It takes taxable income (zvE) as the input. `assess` rounds that amount down to whole euros, applies the tariff, and returns the income tax. It rejects a negative amount ([Inputs](../usage/README.md#inputs), [Errors](../usage/README.md#errors)).

</div>

## The basic allowance (Grundfreibetrag)

The basic allowance is the part of income that is not taxed at all, because it covers the basic cost of living.

{{#include assets/charts/zero-zone.svg}}

The chart marks the taxable income up to which the tax stays at zero: 12 355 € for a single filer and 24 711 € for a married couple.

For 2026 the basic allowance is 12 348 €, and a single filer pays no income tax up to that amount. The rule is in [§ 32a Abs. 1 Satz 2 Nr. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html).

The basic allowance is not a deduction from income. It is the end of the zero zone, which is the first zone of the tariff. The next euro starts the next zone, but its tax rounds down to 0 €. So the zero zone stays at 0 € a few euros longer, up to 12 355 € for a single filer. The next section explains why the cents matter here.

The basic allowance is set again every year. The chart shows it for each assessment year from 2003 to 2026. It rises slowly until 2021 and faster after 2022, from 9 744 € in 2021 to 11 784 € in 2024.

{{#include assets/charts/allowance-history.svg}}

Why does it rise? The basic allowance is the amount a person needs to live on without paying income tax, the subsistence minimum (Existenzminimum). When prices rise, the same life costs more, so the untaxed amount rises too. The government reviews this in its subsistence minimum report (Existenzminimumbericht), and the law is changed to match. The [2024 report](https://github.com/kondominium/fiskus/raw/main/docs/references/2024-bt-drs-existenzminimumbericht-20-04443.pdf) is one example. The 2023 figure came from the 2022 Inflation Compensation Act (Inflationsausgleichsgesetz).

The yearly amounts are written into the statute itself. The basic allowance and the tariff zones are in [§ 32a EStG](https://www.gesetze-im-internet.de/estg/__32a.html), which also says from which assessment period each figure applies.

<div class="info-box">

{{#include assets/callouts/book-open.svg}}

**Did you know?** In 1990 the basic allowance was 5 616 DM, about 2 872 € at the fixed rate of 1 € = 1.95583 DM. For 2026 it is 12 348 €, more than four times the 1990 figure in nominal terms. Source: [Grundfreibetrag (Deutschland), de.wikipedia.org](https://de.wikipedia.org/wiki/Grundfreibetrag_(Deutschland)).

</div>

<div class="info-box">

{{#include assets/callouts/calculator.svg}}

**Example Scenario:** Lena's taxable income of 12 348 € is exactly the basic allowance, and she pays 0 €. At 24 696 € she pays 2 770 €.

</div>

<div class="info-box">

{{#include assets/callouts/lightbulb.svg}}

**How fiskus deals with that:** `basic_allowance_eur` returns the basic allowance of a year in whole euros, and `assess` returns 0 € for any taxable income up to it ([Quick start](../usage/README.md#quick-start)).

</div>

## Rounding down (Abrundung)

**The cents are dropped, never rounded up.** Two incomes that differ only in cents pay the same tax.

The tax law works with whole euros. Taxable income is rounded down to whole euros first ([§ 32a Abs. 1 Satz 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). The zone formula then gives a fraction of euros, and the tax is rounded down once more, on the result of the formula ([§ 32a Abs. 1 Satz 6 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)).

| Taxable income | Cents cut off | Taken as | Income tax |
|---|---|---|---|
| 24 696.00 € | 0.00 € | 24 696 € | 2 770 € |
| 24 696.25 € | 0.25 € | 24 696 € | 2 770 € |
| 24 696.50 € | 0.50 € | 24 696 € | 2 770 € |
| 24 696.99 € | 0.99 € | 24 696 € | 2 770 € |

<div class="info-box">

{{#include assets/callouts/calculator.svg}}

**Example Scenario:** Mara and Jonas have 61 740.75 € of joint taxable income. The 0.75 € is dropped, so the tariff works with 61 740 €. Their joint income tax is 8 926 €, the same as at exactly 61 740 €.

</div>

<div class="info-box">

{{#include assets/callouts/lightbulb.svg}}

**How fiskus deals with that:** `assess` rounds the taxable income down to whole euros before the tariff applies. Amounts are stored as whole cents, so the rounding is exact ([Inputs](../usage/README.md#inputs)).

</div>

With the cents settled, the next question is whose income the tariff is applied to. For a married couple, that is a question of filing status.

## Who is taxed together (Veranlagungsform)

A married couple's zero zone is twice the single one. The reason is that a couple is taxed on half of its joint income. That raises the question of who is taxed together.

{{#include assets/charts/joint-splitting.svg}}

The solid line is a single filer, and the dashed line is a married couple, both for the same combined taxable income in 2026.

Married spouses and registered partners can be assessed jointly (Zusammenveranlagung, [§ 26b EStG](https://www.gesetze-im-internet.de/estg/__26b.html)), and the splitting tariff (Splittingtarif) applies. Or they can be assessed separately (getrennte Veranlagung). Under the conditions of [§ 26 Abs. 1 EStG](https://www.gesetze-im-internet.de/estg/__26.html), they choose between the two.

The marginal rate shows the effect best. Splitting halves the income, so a couple's marginal rate is the single filer's rate at half the income. At 61 740 € of combined taxable income, a single filer pays 39.2 % on the next euro. A couple pays the rate of a single filer at 30 870 €, which is 28.5 %.

The splitting tariff halves the joint taxable income, applies the tariff to that half, and doubles the result. The joint tax is therefore always an even number of euros:

$$
\text{joint tax} = 2 \times \text{tariff}\left(\left\lfloor \frac{\text{zvE}}{2} \right\rfloor\right)
$$

<div class="info-box">

{{#include assets/callouts/user.svg}}

**Single filer:** One person is taxed alone. The tariff applies to that person's own taxable income, with no halving.

</div>

<div class="info-box">

{{#include assets/callouts/users.svg}}

**Joint filers:** Two spouses are taxed as one unit, on their combined taxable income. The splitting tariff applies.

</div>

<div class="info-box">

{{#include assets/callouts/calculator.svg}}

**Example Scenario:** Lena is a single filer with 24 696 € of taxable income, and she pays 2 770 €. Mara and Jonas are assessed jointly on the same 24 696 €. Half of that is 12 348 €, the basic allowance, so the tariff gives no tax on that half, and their joint income tax is 0 €.

</div>

<div class="info-box">

{{#include assets/callouts/lightbulb.svg}}

**How fiskus deals with that:** `assess` takes a `TaxYear` and a `FilingUnit` with its `FilingStatus`, and returns the income tax as an `Assessment` ([Inputs](../usage/README.md#inputs)).

</div>

Once it is known whose income is taxed, the tariff turns that income into tax, zone by zone.

## The tariff (Tarif)

**Income tax rises zone by zone.** Above the zero zone, the tax climbs, and the chart shows how fast.

{{#include assets/charts/tax-by-income.svg}}

The curve looks almost straight above 69 879 € of taxable income, where the marginal rate settles at 42 %. It bends at the low end, where the rate climbs, and again at 277 826 €, where the marginal rate steps up to 45 %. The curve is drawn against taxable income (zvE), because the tariff applies to taxable income.

The table gives the figures for selected incomes.

{{#include assets/tables/tax-by-income.md}}

The tariff of [§ 32a Abs. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html) has up to five zones (Tarifzone), starting with the zero zone. Each zone has its own formula, and the rate on each extra euro is higher than in the zone before.

| Zone | Taxable income (zvE) | Tax in euros |
|---|---|---|
| 1 | up to the basic allowance | 0 € |
| 2 | the next euro, up to the start of zone 3 | a quadratic formula in `t` |
| 3 | up to the start of zone 4 | a quadratic formula in `t` |
| 4 | up to the start of zone 5 | a linear formula in `t` |
| 5 | from the start of zone 5 on | a linear formula in `t` |

`x` is the whole-euro taxable income. Each zone measures the income above its base as `t = (x − base) / step`. For zones 2 and 3 the step is 10 000 €, so `t` counts units of 10 000 €. The base is 12 348 € for zone 2 and 17 799 € for zone 3. For zones 4 and 5 the step is one euro, so `t` is `x`. For 2026, zone 2 starts at 12 349 €, zone 3 at 17 800 €, zone 4 at 69 879 € and zone 5 at 277 826 €.

The numbers in each formula change from year to year. Some years have fewer zones. The statute of the year gives them.

$$
t = \frac{x - \text{base}}{\text{step}}, \qquad \text{tax} = a\,t^2 + b\,t + c
$$

Linear zones have a = 0.

<div class="info-box">

{{#include assets/callouts/calculator.svg}}

**Example Scenario:** Lena's taxable income is 61 740 € in 2026, and her income tax is 14 909 €. Mara and Jonas, assessed jointly with the same 61 740 €, pay 8 926 €. The joint figure is lower, because the splitting tariff applies the zones to half the income.

</div>

<div class="info-box">

{{#include assets/callouts/lightbulb.svg}}

**How fiskus deals with that:** `tariff_zones` returns the zones of a year with their coefficients, and `explain` returns the zone that applies, with the unrounded tax and the citation of the year's tariff ([Explanations and rates](../usage/README.md#explanations-and-rates)).

</div>

The tax is now known. What remains is how it relates to income: the share that goes to tax, and what one more euro costs.

## Rates and surcharge (Steuersatz und Solidaritätszuschlag)

**Each rate answers a different question.**

{{#include assets/charts/tariff-rates.svg}}

The solid line is the marginal rate, and the dashed line is the average rate, the same as in the opening chart.

{{#include assets/charts/progressive-rates.svg}}

The chart marks the zone rate: the marginal rate at the first euro of each zone above the zero zone. Each zone opens at a higher rate than the zone before.

Take an income of 277 826 €, where zone 5 starts. Three rates describe the tax there, and each answers a different question.

- The average tax rate (Durchschnittssteuersatz), also called the effective tax rate, is the income tax divided by the taxable income. It answers: what share of my income goes to tax?
- The marginal tax rate (Grenzsteuersatz) is the tax on the next euro of taxable income. It answers: what does one more euro cost me?
- The zone rate is the marginal rate at the first euro of a zone. For 2026 it starts at 14.0 % in zone 2, at 24.0 % in zone 3, at 42.0 % in zone 4 and at 45.0 % in zone 5. It is the rate a new zone opens with, not the rate paid on all of the income.

<div class="info-box">

{{#include assets/callouts/calculator.svg}}

**Example Scenario:** Lena is a single filer with 277 826 € of taxable income. The tariff gives an income tax of 105 551 €. Her average tax rate is 37.99 %, the share of her income that goes to tax. Her marginal tax rate is 45.00 %, the rate on the next euro.

</div>

$$
\text{average rate} = \frac{\text{income tax}}{\text{taxable income}}, \qquad \text{marginal rate} = \frac{d\,\text{tax}}{d\,\text{taxable income}}
$$

The marginal rate is the derivative of the zone's formula. It rises continuously in zones 2 and 3, and it is flat in zones 4 and 5, at 42 % and 45 %. The average tax rate rises with income, because the zones at the top are taxed at higher rates.

The tax does not jump at a zone boundary, because the zones join. The marginal rate changes there only because the formula changes. The marginal rate of the zero zone is zero.

### The solidarity surcharge

The solidarity surcharge (Solidaritätszuschlag) is added on top of the income tax, so it is the last step of the calculation.

{{#include assets/charts/solidarity-surcharge.svg}}

For 2026 the surcharge is zero up to a free threshold (Freigrenze) of 20 350 € of income tax. Above it, a mitigation zone (Milderungszone) applies first, so the surcharge rises gently. The line bends at about 37 839 € of income tax, where the full 5.5 % rate takes over.

The Solidarity Surcharge Act (Solidaritätszuschlaggesetz 1995, SolZG) sets the rules:

- Below the free threshold of [§ 3 Abs. 3 SolZG](https://www.gesetze-im-internet.de/solzg_1995/__3.html), the surcharge is zero. For 2026 the threshold is 20 350 €, or 40 700 € for joint filers.
- Above it, the surcharge is 5.5 % of the income tax ([§ 4 SolZG](https://www.gesetze-im-internet.de/solzg_1995/__4.html)).
- In the mitigation zone, the surcharge is capped at 11.9 % of the amount by which the income tax exceeds the threshold. This keeps the transition smooth.

The next chart shows the same rules in the marginal rate. The gap between the two lines is the surcharge's effect. The surcharge starts at 74 969 € of taxable income, and the full 5.5 % applies from 116 607 €.

{{#include assets/charts/marginal-rate-solidarity.svg}}

<div class="info-box">

{{#include assets/callouts/calculator.svg}}

**Example Scenario:** Lena's income tax is 105 551 €. That is far above the single threshold of 20 350 €, so the full 5.5 % applies, and the surcharge is about 5 805 €.

</div>

$$
\text{surcharge} = 5.5\,\% \times \text{income tax}
$$

<div class="info-box">

{{#include assets/callouts/book-open.svg}}

**Did you know?** The surcharge was first levied in 1991 to help pay for German reunification. It was suspended for 1993 and 1994, came back in 1995, and was cut to 5.5 % in 1998. Since 2021, a higher free threshold and the mitigation zone have meant that most payers no longer pay it in full. Sources: [Solidaritätszuschlag, de.wikipedia.org](https://de.wikipedia.org/wiki/Solidarit%C3%A4tszuschlag) and the statute above.

</div>

<div class="info-box">

{{#include assets/callouts/lightbulb.svg}}

**How fiskus deals with that:** `explain` returns `average_rate` and `marginal_rate` as fractions of one, and `solidarity_surcharge` returns the surcharge in cents, or `None` for a year without a threshold ([Explanations and rates](../usage/README.md#explanations-and-rates), [Solidarity surcharge](../usage/README.md#solidarity-surcharge)).

</div>

That completes the calculation from the year to the surcharge. Not everything in the law is covered yet, and the next section says what is still missing.

## Future

fiskus is not finished. It computes income tax from taxable income, and it leaves out the parts of the law that come before taxable income and the amounts that change the tax later. Not yet in fiskus:

- Allowances (Freibeträge) and special expenses (Sonderausgaben), which make up total income and taxable income.
- Child allowances and child benefit (Kindergeld).
- Parental allowance (Elterngeld), which is not taxable income but can raise the rate on taxable income through the progression reservation (Progressionsvorbehalt). Its rules are in the [Federal Parental Allowance Act](https://www.gesetze-im-internet.de/beeg/__2.html).
- Real estate depreciation (Immobilien-AfA).
- Church tax (Kirchensteuer), which each state sets.

fiskus will keep providing more value over time. It will first focus on keeping the engine tight and on building the agentic factory, a framework that builds trust in what it does. It will release a new tax year automatically each year, and walk toward covering more of the law. Contributions are welcome!

## How fiskus thinks about the numbers

The last section shows how fiskus turns these rules into code. Each box links back to the part of the guide that explains it.

<div class="info-box">

{{#include assets/callouts/coins.svg}}

**Exact money:** Amounts are whole cents, never floating point numbers. See [Taxable income](#taxable-income-zu-versteuerndes-einkommen-zve) and [Rounding down](#rounding-down-abrundung).

</div>

<div class="info-box">

{{#include assets/callouts/book-open.svg}}

**Every figure has a source:** each value names its statute section, its source and the date it was retrieved. See [The tax year](#the-tax-year-veranlagungszeitraum).

</div>

<div class="info-box">

{{#include assets/callouts/calendar.svg}}

**One file per year:** each tax year has its own rules file, so a new year is a new file and the calculation stays the same. See [The basic allowance](#the-basic-allowance-grundfreibetrag).

</div>

<div class="info-box">

{{#include assets/callouts/shield-check.svg}}

**Errors, not guesses:** a negative income is an error, not a zero tax, and fiskus does not estimate figures for a year it does not list. See [Errors](../usage/README.md#errors).

</div>

<div class="info-box">

{{#include assets/callouts/code.svg}}

**Code references:** `assess` returns the income tax, `explain` returns the steps and the rates, and `basic_allowance_eur` and `tariff_zones` return the parameters. See [Quick start](../usage/README.md#quick-start) and [Explanations and rates](../usage/README.md#explanations-and-rates).

</div>

fiskus is an independent reference engine. It is not official BMF or ELSTER software, and it is **not tax advice**.

The sources below are the statutes and documents behind each step.

## Sources

- [Income Tax Act (Einkommensteuergesetz, EStG)](https://www.gesetze-im-internet.de/estg/): the statute behind the tax calculation.
- [Solidarity Surcharge Act (Solidaritätszuschlaggesetz 1995, SolZG)](https://www.gesetze-im-internet.de/solzg_1995/): the statute behind the surcharge, its rate and its free threshold.
- [Solidaritätszuschlag (de.wikipedia.org)](https://de.wikipedia.org/wiki/Solidarit%C3%A4tszuschlag): the surcharge's history. It is a secondary source, so the statutes above are the reference for the rules.
- [Tax Development Act (Steuerfortentwicklungsgesetz, SteFeG)](https://github.com/kondominium/fiskus/raw/main/docs/references/2024-bgbl-steuerfortentwicklungsgesetz-I-449.pdf): the 2024 act that sets the surcharge thresholds for 2024, 2025 and 2026.
- [Subsistence minimum report (Existenzminimumbericht)](https://github.com/kondominium/fiskus/raw/main/docs/references/2024-bt-drs-existenzminimumbericht-20-04443.pdf): the government's regular report on the subsistence minimum. The 2024 report is linked.
- [Withholding guidance (Programmablaufplan, PAP)](https://github.com/kondominium/fiskus/raw/main/docs/references/2026-bmf-pap-2026.pdf): the Federal Ministry of Finance's yearly guidance for employers' withholding. Several year files in `rules/` cite it.
- [Parental Allowance Act (Bundeselterngeld- und Elternzeitgesetz, BEEG)](https://www.gesetze-im-internet.de/beeg/): the rules for parental allowance.
- [ELSTER](https://www.elster.de/): the official online tax portal.
- [Lucide](https://lucide.dev): the icons, under their own license.
