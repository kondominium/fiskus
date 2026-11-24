# Basic allowance zone (Grundfreibetrag), § 32a EStG

The basic allowance (Grundfreibetrag) is not a deduction from income. It is the first zone of the tariff: tax is zero for taxable income up to the threshold.

## Statute

§ 32a Abs. 1 Satz 1 EStG and § 32a Abs. 1 Satz 2 Nr. 1 EStG (source: https://www.gesetze-im-internet.de/estg/__32a.html, retrieved 2026-11-22):

> Die tarifliche Einkommensteuer bemisst sich nach dem auf volle Euro abgerundeten zu versteuernden Einkommen. Sie beträgt ab dem Veranlagungszeitraum 2026 [...] jeweils in Euro für zu versteuernde Einkommen 1. bis 12 348 Euro (Grundfreibetrag): 0

§ 32a Abs. 5 EStG (splitting procedure, Splitting-Verfahren):

> Bei Ehegatten, die nach den §§ 26, 26b zusammen zur Einkommensteuer veranlagt werden, beträgt die tarifliche Einkommensteuer [...] das Zweifache des Steuerbetrags, der sich für die Hälfte ihres gemeinsam zu versteuernden Einkommens nach Absatz 1 ergibt.

## Rules

Let $z$ be the taxable income (zu versteuerndes Einkommen) in cents. Let $e = \lfloor z / 100 \rfloor$ be the whole-euro amount. The statute rounds down before any tariff step.

Single, basic tariff (Grundtarif, § 26a EStG):

$$
b = e
$$

Joint, splitting tariff (Splittingtarif, § 26b EStG with § 32a Abs. 5 EStG):

$$
b = \left\lfloor \frac{e}{2} \right\rfloor
$$

The zone applies when $b \le G$, where $G$ is the basic allowance for the assessment year. For VZ 2026, $G = 12\,348$. The values for all years 2003 to 2026 are in `rules/<year>.toml` and are exposed as `basic_allowance_eur(year)`. In that zone the tariff tax is $0$, and for joint filing $2 \cdot 0 = 0$.

Outside the zone, the code returns an error. The next tariff zones are not implemented yet.

## Boundaries (VZ 2026)

| Filing | Last zero-tax taxable income (whole euros) | First taxable income outside the zone |
|---|---|---|
| Single | 12 348 | 12 349 |
| Joint | 24 697 | 24 698 |

The joint boundary is 24 697, not 24 696. The half of 24 697 is 12 348.50, which rounds down to 12 348, still inside the zone. A joint boundary of 24 696 would only follow if the half were not rounded down before the zone check, which § 32a Abs. 1 Satz 1 does not allow.

## Not covered here

- Zones 2 to 5 of § 32a Abs. 1 EStG (tax above the basic allowance).
- How taxable income (zu versteuerndes Einkommen) is computed from income (§ 2 EStG).
- The solidarity surcharge (Solidaritätszuschlag) and church tax (Kirchensteuer).
- The exceptions named in § 32a Abs. 1 Satz 2 EStG: § 32b EStG, § 32d EStG, § 34 EStG and § 34a EStG.
