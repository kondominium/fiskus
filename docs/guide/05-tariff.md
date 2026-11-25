{{#include assets/icons/tariff.svg}}

# The tariff

**Income tax rises with taxable income, zone by zone.** The tariff (Tarif) of [§ 32a Abs. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html) has a zero zone, the basic allowance (Grundfreibetrag), and then progressive zones. Each zone has its own formula, and the rate on each extra euro is higher than in the zone before.

## The zones

| Zone | Taxable income (zvE) | Tax in euros |
|---|---|---|
| 1 | up to the basic allowance | 0 € |
| 2 | the next euro up to the start of zone 3 | a quadratic formula in `y` |
| 3 | up to the start of zone 4 | a quadratic formula in `z` |
| 4 | up to the start of zone 5 | a linear formula in `x` |
| 5 | from the start of zone 5 on | a linear formula in `x` |

`x` is the whole-euro taxable income. `y` and `z` are the parts of it above the start of the zone, scaled by 10 000. Zone 2 uses `y = (x − basic allowance) / 10 000`, and zone 3 uses `z = (x − start of zone 3 + 1) / 10 000`. The numbers in each formula change from year to year, and some years have fewer zones. The statute of the year gives them.

In fiskus each zone is a `ZoneStep`. `tariff_zones` returns the zones of a year with their coefficients, and the formula is `a·t² + b·t + c`, with `t = (x − base) / step`. Linear zones have `a = 0` and a step of one euro. The coefficients of every year are in [Formulas: tariff](../formulas/tariff.md).

### The rate steps up zone by zone

The progressive zones are what make the tax progressive. The chart shows the marginal rate at the first euro of each zone for the latest year: the rate on the next euro, where that zone starts.

{{#include assets/charts/progressive-rates.svg}}

## What income costs in tax

The next chart shows the income tax of a single filer from 1 € to 300 000 € for the latest year. Each point is the tax that `assess` returns for that taxable income. The table gives the same figures for selected incomes, together with the average and the marginal rate.

{{#include assets/charts/tax-by-income.svg}}

{{#include assets/tables/tax-by-income.md}}

The average rate is the tax divided by the income. It rises with income, because the zones at the top are taxed at higher rates. The marginal rate is the rate on the next euro, and it is the one to look at when a raise is in question.

## Rounding

The tax is computed on the whole-euro taxable income. It is rounded down once, at the end of the zone, not at each step ([§ 32a Abs. 1 Satz 6 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). fiskus keeps the exact fraction until that point, so no rounding error builds up.

Because of this rule, the tax changes by at most one euro at each zone start. The test suite checks that for every listed year.

## Joint filers

For joint filers, the tariff is applied to half of the joint taxable income, and the result is doubled ([§ 32a Abs. 5 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). The half is rounded down before the tariff applies. The joint tax is therefore always even, in whole euros.

## Reading an explanation

`explain` returns the zone that applies, the unrounded tax as a fraction, the rounded tax, the two rates, and the citation of the year's tariff. A year without zones has no source in the explanation, because fiskus has no tariff for it to cite.

## Not yet in fiskus

- The special cases of § 32a Abs. 1 Satz 2 EStG, such as the progression reservation (Progressionsvorbehalt) and the tariff for widowed filers.
- The reduction for the splitting procedure in the year of a death (§ 32a Abs. 6 EStG).
- The rule that taxable income comes from income ([§ 2 EStG](https://www.gesetze-im-internet.de/estg/__2.html)).
