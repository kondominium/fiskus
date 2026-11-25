{{#include assets/icons/rates.svg}}

# Rates and surcharge

## Average and marginal rate

The tariff implies two rates, and they answer different questions. The first is the **average tax rate**, also called the effective tax rate (Durchschnittssteuersatz). It is the income tax divided by the taxable income, so it answers: what share of my income goes to tax? The second is the **marginal tax rate** (Grenzsteuersatz). It is the tax on the next euro of taxable income, so it answers: what does one more euro of income cost me?

The marginal rate is the derivative of the zone's formula. It starts at the rate of the first progressive zone and rises within each zone. The tax itself does not jump at a zone boundary, because the zones join. The marginal rate changes at a boundary only because the formula changes there. The marginal rate of the zero zone is zero.

{{#include assets/charts/tariff-rates.svg}}

`explain` reports both rates as `average_rate` and `marginal_rate`, as fractions of one. A zero income has an average rate of zero. A joint filer's marginal rate is the rate of the single filer on half the joint income, because the splitting tariff applies the same formula to that half.

## Solidarity surcharge

The solidarity surcharge (Solidaritätszuschlag) is charged on the income tax. The Solidarity Surcharge Act (Solidaritätszuschlaggesetz 1995, SolZG) sets it:

- Below a free threshold (Freigrenze) of § 3 Abs. 3 SolZG, the surcharge is zero. The threshold is set for each year, and it is higher for joint filing.
- Above it, the surcharge is 5.5 % of the income tax (§ 4 SolZG). In the mitigation zone it is capped at 11.9 % of the amount by which the income tax exceeds the threshold, so the transition is smooth.

{{#include assets/charts/solidarity-surcharge.svg}}

`solidarity_surcharge` takes the year, the filing status and the income tax, and returns the surcharge in cents. `solidarity_free_threshold_eur` returns the threshold, and `solidarity_source` returns its citation and source. The threshold comes from the rule file of the year, so each year's figure is checked against its own statute.

**Years without a threshold.** fiskus has the threshold for the years whose statute is archived in `docs/references`. For a year without one, `solidarity_surcharge` returns `None`, not a guessed figure. Each year's threshold is added with its source when the statute is archived.

## Not yet in fiskus

- Church tax (Kirchensteuer), which each state sets for its members.
- The thresholds of the years before the archived ones.
