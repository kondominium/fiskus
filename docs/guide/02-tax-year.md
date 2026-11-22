{{#include assets/icons/tax-year.svg}}

# The tax year

Income tax is assessed for one calendar year, called the assessment period (Veranlagungszeitraum, VZ). Each year's bill is worked out from that year's rules, so every calculation in fiskus names its year.

## Where the yearly rules come from

Income tax is set by law. The [Income Tax Act (Einkommensteuergesetz, EStG)](https://www.gesetze-im-internet.de/estg/) is that law. Parliament (Bundestag) changes it, and the changes are published in the Federal Law Gazette (Bundesgesetzblatt, BGBl.).

The yearly amounts, such as the basic allowance, are written into the law itself. The tariff is in [§ 32a EStG](https://www.gesetze-im-internet.de/estg/__32a.html), and the figures say from which year they apply. For example, the basic allowance is stated "from the assessment period 2026" (ab dem Veranlagungszeitraum 2026).

The Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) administers the law. It publishes guidance and tables that apply these figures, and the tax office (Finanzamt) uses them to make each assessment. The consolidated text of every law is free to read at gesetze-im-internet.de, which is where fiskus takes its figures from. The official ELSTER portal is at [elster.de](https://www.elster.de/).

**Every figure in fiskus names its citation, its source, and the date it was checked.** You can compare any of them with the law yourself.

## When the next year's rules appear

The guide currently assumes that the rules for the next year appear in November and December. The illustration below shows those two months. **It is an illustration, not a measurement.** We have not yet checked these dates against sources, and we will replace it with dated data once we have.

{{#include assets/illustrations/publication-months.svg}}

## What fiskus supports today

fiskus has one year: VZ 2026, given as `TaxYear::Y2026`. There is no other year to pass in, so fiskus does not estimate the amounts for other years.

## Planned years

We plan to add more years. Each year gets its own parameter file in `rules/`, checked line by line against that year's statute, and only then becomes a `TaxYear`. When the law changes for a year that is already added, the parameter file and this page change together.
