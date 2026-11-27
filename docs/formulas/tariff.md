# Tariff (Tarif), § 32a EStG

The tariff turns taxable income (zu versteuerndes Einkommen, zvE) into income tax. It has a zero zone, the basic allowance (Grundfreibetrag, see [the basic allowance](grundfreibetrag.md)), and progressive zones after it. The formulas are the same in every year. The thresholds and coefficients differ by year, and they are listed below, read from `rules/`.

## Statute

§ 32a Abs. 1 Satz 2 EStG sets the tax for each assessment year (Veranlagungszeitraum, VZ) from the year the figures apply. The text has this form, where the angle brackets stand for the year's figures:

> Die tarifliche Einkommensteuer bemisst sich nach dem auf volle Euro abgerundeten zu versteuernden Einkommen. Sie beträgt ab dem Veranlagungszeitraum `<Y>` [...] jeweils in Euro für zu versteuernde Einkommen 1. bis `<G>` Euro (Grundfreibetrag): 0; 2. von `<F2>` Euro bis `<F3 − 1>` Euro: `(<a2> · y + <b2>) · y`; 3. von `<F3>` Euro bis `<F4 − 1>` Euro: `(<a3> · z + <b3>) · z + <c3>`; 4. von `<F4>` Euro bis `<F5 − 1>` Euro: `<b4> · x − <c4>`; 5. von `<F5>` Euro an: `<b5> · x − <c5>`. Die Größe „y“ ist ein Zehntausendstel des den Grundfreibetrag übersteigenden Teils des auf einen vollen Euro-Betrag abgerundeten zu versteuernden Einkommens. Die Größe „z“ ist ein Zehntausendstel des `<F3 − 1>` Euro übersteigenden Teils des auf einen vollen Euro-Betrag abgerundeten zu versteuernden Einkommens. Die Größe „x“ ist das auf einen vollen Euro-Betrag abgerundete zu versteuernde Einkommen. Der sich ergebende Steuerbetrag ist auf den nächsten vollen Euro-Betrag abzurunden.

Source: [§ 32a EStG](https://www.gesetze-im-internet.de/estg/__32a.html). The figures of each year and their archived statute are in the table below, and in `docs/references`. Some years have fewer zones, because their statute has fewer. Their zones are in the table too.

The statute writes decimals with a comma. fiskus writes them with a point.

## Zones

For a zone with start `F` and base `B`, the tax on the whole-euro basis `x` is

$$
T(x) = \left\lfloor a\,t^2 + b\,t + c \right\rfloor, \qquad t = \frac{x - B}{s}
$$

with the zone's coefficients `a`, `b`, `c` and step `s`. A quadratic zone has `s = 10 000`, and its base is the euro before its start, `B = F − 1`. A linear zone has `a = 0` and `s = 1`. The zone is the last one whose start is at or below `x`. Below the start of zone 2 the tax is zero.

The coefficients per year are listed below. A coefficient is in euros, and `t` is a number of steps. fiskus stores each coefficient in hundredths of a euro, so `quadratic_h = 91451` means `a = 914.51`. The exact form uses integers only:

$$
T(x) = \left\lfloor \frac{a_h\,d^2 + b_h\,d\,s + c_h\,s^2}{100\,s^2} \right\rfloor, \qquad d = x - B
$$

The numerator is one integer, and the floor is taken once, at the end of the zone, as § 32a Abs. 1 Satz 6 EStG requires. Every product is checked. An overflow is `None`, which surfaces as `Error::OutsideImplementedRange`, never as a wrong tax.

## Rounding

Taxable income is rounded down to whole euros before the tariff applies (§ 32a Abs. 1 Satz 1 EStG). The tax is rounded down once, at the end of the zone. Because the zones join, the tax changes by at most one euro at each zone start. VZ 2003 has the table step of § 32a Abs. 2 EStG, which holds the tax flat over blocks of 36 € and moves it at each block edge. See [table steps](table-steps.md).

## Joint filers

For joint filers, the tariff is applied to half of the joint taxable income, rounded down, and the result is doubled (§ 32a Abs. 5 EStG):

$$
T_{\text{joint}}(z) = 2\,T\!\left(\left\lfloor \frac{z}{2} \right\rfloor\right)
$$

The joint tax is therefore always a whole number of euros, and always even.

## Rates

With the unrounded tax `U(x)` (the fraction before the floor), the average rate, also called the effective tax rate, is `T(x) / x` for `x > 0`. The marginal rate is the tax on the next euro, which is the derivative of the zone's formula:

$$
m(x) = \frac{2\,a\,t + b}{s}
$$

`explain` reports both as ratios of one. They are `f64`, and they never feed back into a tax. The rates are described on the [rates and surcharge](../guides/how-income-taxes-are-calculated-in-germany.md#rates-and-surcharge-steuersatz-und-solidaritätszuschlag) section.

## Coefficients by year

The tables are generated from `rules/` by `make charts`, so they match the code. Do not edit them by hand.

{{#include tariff-coefficients.md}}

## What the tests check

The tests check behaviour for every listed year, not fixed figures. See [How fiskus is tested](../internal/testing/README.md).

- Zero tax up to the basic allowance, with any cents part.
- Tax never falls as income rises, for single and joint filers.
- Joint tax is twice the tax on half the income, rounded down.
- Each euro of income adds at most one euro of tax, since every marginal rate is below one.
- The tax changes by at most one euro at each zone start, so the zones join.
- The marginal rate equals the numerical derivative of the unrounded tax, and a joint filer's rate equals the single rate on half the income.
- Each explanation's zone contains its basis, and its unrounded tax floors to the rounded tax.
- Any `i128` income returns `Ok` or `Err` and never panics.

The conformance layer checks the figures of every year against the BMF calculator. See [conformance](../internal/testing/README.md).

## Not yet in fiskus

- The special cases of § 32a Abs. 1 Satz 2 EStG, such as the progression reservation (Progressionsvorbehalt).
- The tariff for a surviving spouse (§ 32a Abs. 6 EStG).
- The rule that taxable income comes from income (§ 2 EStG).
