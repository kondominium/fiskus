{{#include assets/icons/tax-year.svg}}

# The tax year

Income tax (Einkommensteuer) is assessed for one calendar year, called the assessment period (Veranlagungszeitraum, VZ). Each assessment uses the rules of its own year, so every calculation in fiskus names its year. `TaxYear::Y<year>` is the year, and `TaxYear::year()` gives the calendar number.

## Where the yearly rules come from

The [Income Tax Act (Einkommensteuergesetz, EStG)](https://www.gesetze-im-internet.de/estg/) sets the rules. Parliament (Bundestag) changes it, and each change is published in the Federal Law Gazette (Bundesgesetzblatt, BGBl.).

The yearly amounts are written into the law. The basic allowance (Grundfreibetrag) and the tariff zones are in [§ 32a EStG](https://www.gesetze-im-internet.de/estg/__32a.html). The law states from which assessment period each figure applies.

The Federal Ministry of Finance (Bundesministerium der Finanzen, BMF) administers the law. Its withholding guidance (Programmablaufplan, PAP) applies the same figures for the tax office (Finanzamt), and fiskus checks its figures against that guidance as well.

**Every figure in fiskus names its citation, its source, and the date it was retrieved.** Each one can be compared with the law. The source files are in `docs/references`.

## What fiskus supports

fiskus lists the assessment years in `TaxYear::ALL`, and `TaxYear::from_year` returns the variant for a calendar year. A year outside that list is not supported, so fiskus does not estimate its figures.

Each listed year has a basic allowance. A year with tariff zones also has its tariff. A year without zones has only its basic allowance, and an income above that allowance is reported as outside the implemented range. The [roadmap](../roadmap.md) says which years still lack zones.

## Adding a year

Each year has its own parameter file in `rules/`, checked against that year's sources. When the law changes for a year that is already listed, the parameter file and the guide change together.
