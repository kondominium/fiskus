# Basic allowance (Grundfreibetrag), § 32a EStG

The basic allowance is the first zone of the tariff, and its tax is zero. It is the amount in `rules/<year>.toml` under `[basic_allowance]`, and `basic_allowance_eur(year)` returns it.

## Rule

Let `G` be the basic allowance for the assessment year, and let `x` be the taxable income in whole euros. For a single filer, the zone of zero tax is:

$$
x \le G \quad\Rightarrow\quad T(x) = 0
$$

For a joint filer with joint taxable income `z`, the zone uses the half of `z`, rounded down, against `G`:

$$
\left\lfloor \frac{z}{2} \right\rfloor \le G \quad\Rightarrow\quad T_{\text{joint}}(z) = 0
$$

The zone is the tax zone and not a deduction. Income is not reduced by `G`.

## The zero-tax range runs past G

Tax is rounded down to whole euros at the end of the zone, so the first few euros above `G` still pay no tax. The zero-tax range is the set of incomes where the rounded-down tax is zero, and it is longer than the zone. The boundary is where the first zone's formula reaches one euro.

Take a sample allowance `G = 10 000` and a sample first zone of the statute's form, `(a·y + b)·y` with `y = (x − G) / 10 000`, `a = 1 000` and `b = 1 400`. For `d = x − G = 7`, `y = 0.0007` and the tax is `(1 000 · 0.0007 + 1 400) · 0.0007 ≈ 0.98 €`, which rounds down to 0 €. For `d = 8`, the tax is about 1.12 €, which rounds down to 1 €. So the zero-tax range runs 7 euros past the allowance in this sample. These numbers are sample data, not a statute.

The real boundaries of a year follow from its zones, and `explain` shows the zone and the unrounded tax for any income.

## Joint filers

The splitting tariff applies the zone to half of the income. The joint zero zone is therefore about twice the single zone, plus the few euros of rounding. The guide's [basic allowance section](../guide/README.md#the-basic-allowance-grundfreibetrag) shows the latest year's figures.
