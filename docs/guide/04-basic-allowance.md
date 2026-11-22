{{#include assets/icons/basic-allowance.svg}}

# The basic allowance

**The first part of the tariff is zero.** For a single filer, taxable income up to 12 348 € pays no income tax ([§ 32a Abs. 1 Satz 2 Nr. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)).

The basic allowance (Grundfreibetrag) is not a deduction from income. It sets where the zero zone of the tariff ends, and the zone is the first step of the tariff.

For joint filers, the splitting tariff takes half of the joint taxable income, applies the tariff to that half, and doubles the result ([§ 32a Abs. 5 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). Half of 24 697 € is 12 348.50 €, which rounds down to 12 348 €, still inside the zone. So the zero zone runs to 24 697 € of joint taxable income.

{{#include assets/charts/zero-zone.svg}}

Above the zone, `assess` returns `Error::OutsideImplementedRange`. fiskus does not estimate the tax there, because the next zones are not implemented yet.

## How the amount has changed

The amount is set by law for each assessment period ([§ 32a Abs. 1 Satz 2 Nr. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)), so it changes from year to year. The chart shows the amount for each period from 2006 to 2026, and the change from the period before. Periods from 2002 to 2005 are not in fiskus yet.

{{#include assets/charts/allowance-history.svg}}

- Until 2021 the yearly rises were between 118 € and 336 €. Some periods had no change: 2007 and 2008 stayed at 7 664 €, and 2011 and 2012 stayed at 8 004 €.
- Two amounts were set after their period had started. The 2015 amount was passed in July 2015 for that year. The 2024 amount was passed in December 2024 with effect from 1 January 2024.

### Why the amount rose in 2022 to 2024

The three largest rises came from inflation, through different routes.

- **2022: 603 €, to 10 347 €.** The Tax Relief Act 2022 (Steuerentlastungsgesetz 2022, May 2022) raised the 2022 amount retroactively. The government said the price rises were unexpectedly large, and the tax relief was its quick response.
- **2023: 561 €, to 10 908 €.** The government's 14th report on the subsistence minimum (BT-Drs. 20/4443) calculated 10 908 € for 2023, while the amount then in force was 10 347 €. The Inflation Compensation Act (Inflationsausgleichsgesetz, InflAusG, December 2022) closed that gap. It also offset cold progression, the extra tax from inflation alone, by moving the other tariff thresholds to the right.
- **2024: 876 €, to 11 784 €.** The subsistence minimum is built from the basic social-assistance rates, which follow prices. For 2024 the 14th report expected those rates to rise by about 6 to 8 percent. It calculated 11 472 € for a single person, and the act on the subsistence minimum 2024 (Gesetz zur steuerlichen Freistellung des Existenzminimums 2024) set the basic allowance 312 € higher, at 11 784 €.
