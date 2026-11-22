{{#include assets/icons/tariff.svg}}

# The tariff

**Income tax rises with taxable income, in five zones.** For VZ 2026, fiskus computes all five zones of the tariff ([§ 32a Abs. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). The same five zones apply to every year from VZ 2006 to 2026, with that year's thresholds and rates. The first zone is zero. Each later zone has its own formula, and the rate on each extra euro is higher than in the zone before.

| Taxable income (zvE), single filer | Tax in euros, with x the whole-euro taxable income |
|---|---|
| up to 12 348 € | 0 € (the basic allowance, Grundfreibetrag) |
| 12 349 € to 17 799 € | (914.51 • y + 1 400) • y, with y = (x − 12 348) / 10 000 |
| 17 800 € to 69 878 € | (173.10 • z + 2 397) • z + 1 034.87, with z = (x − 17 799) / 10 000 |
| 69 879 € to 277 825 € | 0.42 • x − 11 135.63 |
| 277 826 € and more | 0.45 • x − 19 470.38 |

The statute writes decimals with a comma. fiskus writes them with a point here.

The tax is rounded down to whole euros at the end of each zone ([§ 32a Abs. 1 Satz 6 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). Taxable income is also rounded down to whole euros first ([§ 32a Abs. 1 Satz 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)).

For joint filers, the tariff is applied to half of the joint taxable income, and the result is doubled ([§ 32a Abs. 5 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). An odd euro amount is halved and rounded down first.

{{#include assets/charts/tariff-curve.svg}}

Some examples for VZ 2026:

- A single filer with 30 000 € of taxable income pays 4 217 €.
- A joint filer with 100 000 € of joint taxable income pays 21 096 €. Half of it, 50 000 €, pays 10 548 €, and the result is doubled.

The chart above shows the single tariff. Because the tax is rounded down, the first few euros above the basic allowance still pay no tax. A single filer pays 0 € up to 12 355 €, and the first euro of tax is charged at 12 356 €. Joint filers pay 0 € up to 24 711 €.

## Rates

The tariff also implies two rates. The average rate is the income tax divided by the taxable income. The marginal rate is the tax on the next euro, taken from the derivative of the zone's formula. fiskus reports both in `explain` (`average_rate` and `marginal_rate`), and the calculator shows them.

{{#include assets/charts/tariff-rates.svg}}

## Where the numbers come from

The formulas and their thresholds are the statute's. The same zones appear in the withholding program flowchart (Programmablaufplan, PAP) that the Federal Ministry of Finance (BMF) publishes for 2026, and fiskus checked against it. For VZ 2026, fiskus also compared its results with a separate calculation of the same formulas for every whole euro from 0 € to 300 000 €, for single and joint filers. That comparison runs outside the repository. The other years are checked at sample incomes, and their coefficients are in [Formulas: tariff](../formulas/tariff.md).

## Not yet in fiskus

- Years before 2006. VZ 2002 to 2005 are not yet in fiskus.
- The special cases in § 32a Abs. 1 Satz 2 EStG, such as the progression reservation and the lower rate for some incomes.
- The solidarity surcharge (Solidaritätszuschlag) and church tax (Kirchensteuer).
