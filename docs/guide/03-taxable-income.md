{{#include assets/icons/taxable-income.svg}}

# Taxable income

The tariff (Tarif) is applied to taxable income (zu versteuerndes Einkommen, zvE). Two rules apply before the tariff sees the amount.

**Whole euros.** The tax is computed from zvE rounded down to the whole euro ([§ 32a Abs. 1 Satz 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). Cents are dropped, never rounded up. An amount of 1 000.99 € is taken as 1 000 €.

**Exact money.** fiskus stores amounts as whole cents (`Cents`), never as floating point numbers, so the rounding is exact.

{{#include assets/charts/rounding.svg}}

**Today zvE is an input.** fiskus does not yet compute it from income (see the [overview](README.md)). Callers pass the amount in `FilingUnit`, and fiskus rejects a negative amount with `Error::NegativeTaxableIncome`.
