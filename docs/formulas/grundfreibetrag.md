# Basic allowance zone (Grundfreibetrag, § 32a EStG, VZ 2026)

The basic allowance (Grundfreibetrag) is not a deduction from income. It is the first zone of the tariff: tax is zero for taxable income up to the threshold.

## Statute

§ 32a Abs. 1 Satz 1 and 2 Nr. 1 EStG (source: https://www.gesetze-im-internet.de/estg/__32a.html, retrieved 2026-11-22):

> Die tarifliche Einkommensteuer bemisst sich nach dem auf volle Euro abgerundeten zu versteuernden Einkommen. Sie beträgt ab dem Veranlagungszeitraum 2026 [...] jeweils in Euro für zu versteuernde Einkommen 1. bis 12 348 Euro (Grundfreibetrag): 0

§ 32a Abs. 5 EStG (splitting procedure, Splitting-Verfahren):

> Bei Ehegatten, die nach den §§ 26, 26b zusammen zur Einkommensteuer veranlagt werden, beträgt die tarifliche Einkommensteuer [...] das Zweifache des Steuerbetrags, der sich für die Hälfte ihres gemeinsam zu versteuernden Einkommens nach Absatz 1 ergibt.

## Rules

Let $z$ be the taxable income (zu versteuerndes Einkommen) in cents. Let $e = \lfloor z / 100 \rfloor$ be the whole-euro amount. The statute rounds down before any tariff step.

Single (basic tariff, Grundtarif, § 26a):

$$
b = e
$$

Joint (splitting tariff, Splittingtarif, § 26b with § 32a Abs. 5):

$$
b = \left\lfloor \frac{e}{2} \right\rfloor
$$

The zone applies when $b \le 12\,348$. In that zone the tariff tax is $0$, and for joint filing $2 \cdot 0 = 0$.

Outside the zone, the code returns an error. The next tariff zones are not implemented yet.

## Boundaries (VZ 2026)

| Filing | Last zero-tax zvE (whole euros) | First zvE outside the zone |
|---|---|---|
| Single | 12 348 | 12 349 |
| Joint | 24 697 | 24 698 |

The joint boundary is 24 697, not 24 696. The half of 24 697 is 12 348.50, which rounds down to 12 348, still inside the zone. A joint boundary of 24 696 would only follow if the half were not rounded down before the zone check, which § 32a Abs. 1 Satz 1 does not allow.

## Basic allowance by year

Each year's amount is in `rules/<year>/parameters.toml`, with its source in [docs/references](../references/README.md). A year is listed only once its amount has been checked.

The laws in the table:

- Financial Stability Act (Gesetz zur Sicherung von Beschäftigung und Stabilität in Deutschland, StabSiG), BGBl. 2009 I S. 416.
- Act to Reduce Cold Progression (Gesetz zum Abbau der kalten Progression), BGBl. 2013 I S. 283.
- Act to Raise the Basic Allowance and Child Allowances (Gesetz zur Anhebung des Grundfreibetrags, des Kinderfreibetrags, des Kindergeldes und des Kinderzuschlags, KiGAnhG 2015), BGBl. 2015 I S. 1202.
- EU mutual assistance act (Gesetz zur Umsetzung der Änderungen der EU-Amtshilferichtlinie und von weiteren Maßnahmen gegen Gewinnkürzungen und -verlagerungen), BGBl. 2016 I S. 3000.
- Family relief act (Familienentlastungsgesetz), BGBl. 2018 I S. 2210.
- Second family relief act (Zweites Familienentlastungsgesetz), BGBl. 2020 I S. 2616.
- Tax relief act 2022 (Steuerentlastungsgesetz 2022), BGBl. 2022 I S. 749.
- Inflation compensation act (Inflationsausgleichsgesetz, InflAusG), BGBl. 2022 I S. 2230.
- Act on the Existenzminimum 2024 (Gesetz zur steuerlichen Freistellung des Existenzminimums 2024), BGBl. 2024 I Nr. 386.
- Tax Development Act (Steuerfortentwicklungsgesetz, SteFeG), BGBl. 2024 I Nr. 449.

| VZ | Basic allowance | Setting law | Source status |
|---|---|---|---|
| 2006 to 2008 | 7 664 € | not yet traced | official (BMF PAP) |
| 2009 | 7 834 € | StabSiG, BGBl. 2009 I S. 416 | official (BMF PAP) |
| 2010 to 2012 | 8 004 € | StabSiG § 52 Abs. 41, kept by BGBl. 2013 I S. 283 | official (BMF PAP) |
| 2013 | 8 130 € | Act to Reduce Cold Progression, BGBl. 2013 I S. 283 | official (BMF PAP) |
| 2014 | 8 354 € | Act to Reduce Cold Progression, BGBl. 2013 I S. 283 | official (BMF PAP, draft) |
| 2015 | 8 472 € | KiGAnhG 2015, BGBl. 2015 I S. 1202 | official (BMF PAP) |
| 2016 | 8 652 € | KiGAnhG 2015, BGBl. 2015 I S. 1202 | official (BMF PAP) |
| 2017 and 2018 | 8 820 € and 9 000 € | EU mutual assistance act, BGBl. 2016 I S. 3000 | official (BMF PAP) |
| 2019 and 2020 | 9 168 € and 9 408 € | Family relief act, BGBl. 2018 I S. 2210 | official (BMF PAP) |
| 2021 | 9 744 € | Second family relief act, BGBl. 2020 I S. 2616 | official (BMF PAP) |
| 2022 | 10 347 € | Tax relief act 2022, BGBl. 2022 I S. 749 | official (BMF PAP) |
| 2023 | 10 908 € | InflAusG, BGBl. 2022 I S. 2230 | official (BMF PAP) |
| 2024 | 11 784 € | Existenzminimum 2024, Art. 1, BGBl. 2024 I Nr. 386 | primary |
| 2025 | 12 096 € | SteFeG, Art. 1 | primary |
| 2026 | 12 348 € | SteFeG, Art. 2 | primary |

VZ 2002 to 2005 are not yet in fiskus.

## Not covered here

- Zones 2 to 5 of § 32a Abs. 1 (tax above the Grundfreibetrag).
- How zu versteuerndes Einkommen is computed from income (§ 2 EStG).
- Solidaritätszuschlag, Kirchensteuer, and the §§ 32b, 32d, 34 and 34a exceptions named in § 32a Abs. 1 Satz 2.
