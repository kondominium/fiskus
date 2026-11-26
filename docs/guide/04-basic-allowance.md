{{#include assets/icons/basic-allowance.svg}}

# The basic allowance

**The first part of the tariff (Tarif) is zero.** For a single filer, taxable income (zu versteuerndes Einkommen, zvE) up to the basic allowance pays no income tax. The basic allowance (Grundfreibetrag) is written in [§ 32a Abs. 1 Satz 2 Nr. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html). For the latest listed year it is 12 348 €.

The basic allowance is not a deduction from income. It sets where the zero zone of the tariff ends, and the zone is the first step of the tariff.

**Tax is rounded down to whole euros, so the zero-tax range runs a little past the allowance.** The first zone's tax is a little under one euro for the first few euros above the allowance, and rounding down makes it zero. The first euro of tax is due at the first euro where the rounded-down tax reaches one euro.

**Joint filers.** The splitting tariff (Splittingtarif) takes half of the joint taxable income, applies the tariff to that half, and doubles the result ([§ 32a Abs. 5 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). Half of the joint amount is rounded down first, so the joint zero zone is about twice the single one, plus the few euros of rounding.

{{#include assets/charts/zero-zone.svg}}

Above the zero zone, the progressive zones of § 32a Abs. 1 EStG apply. They are on the [tariff](05-tariff.md) page. Each year's allowance is in `rules/` and returned by `basic_allowance_eur`.

The allowance has changed over the years. The chart shows each listed year's figure as one bar. Hover over a bar to see its value.

{{#include assets/charts/allowance-history.svg}}
