{{#include assets/icons/basic-allowance.svg}}

# The basic allowance

**The first part of the tariff (Tarif) is zero.** For a single filer, taxable income (zu versteuerndes Einkommen, zvE) up to 12 348 € pays no income tax (Einkommensteuer) ([§ 32a Abs. 1 Satz 2 Nr. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)).

The basic allowance (Grundfreibetrag) is not a deduction from income. It sets where the zero zone of the tariff ends, and the zone is the first step of the tariff.

For joint filers, the splitting tariff (Splittingtarif) takes half of the joint taxable income, applies the tariff to that half, and doubles the result ([§ 32a Abs. 5 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). Half of 24 697 € is 12 348.50 €, which rounds down to 12 348 €, still inside the zone. So the zero zone runs to 24 697 € of joint taxable income.

{{#include assets/charts/zero-zone.svg}}

Above the zone, `assess` returns `Error::OutsideImplementedRange`. fiskus does not estimate the tax there, because the next zones are not yet in fiskus.
