# Tariff zones (§ 32a EStG, VZ 2006 to 2026)

The tariff turns taxable income (zu versteuerndes Einkommen, zvE) into income tax. Every year from VZ 2006 to 2026 has the same five-zone shape: a zero zone (the basic allowance, Grundfreibetrag, see [Formulas: basic allowance](grundfreibetrag.md)), two progressive zones with a quadratic formula, and two linear zones. The thresholds and coefficients change from year to year, and each year's values are in the table below.

## Statute

§ 32a Abs. 1 Satz 1, 2 and 6 EStG for VZ 2026, as set by the Tax Development Act (Steuerfortentwicklungsgesetz, SteFeG), Art. 2 Nr. 2, BGBl. 2024 I Nr. 449. The copy is in `docs/references`. Source: https://www.gesetze-im-internet.de/estg/__32a.html, retrieved 2026-11-22.

> Die tarifliche Einkommensteuer bemisst sich nach dem auf volle Euro abgerundeten zu versteuernden Einkommen. Sie beträgt ab dem Veranlagungszeitraum 2026 [...] jeweils in Euro für zu versteuernde Einkommen 1. bis 12 348 Euro (Grundfreibetrag): 0; 2. von 12 349 Euro bis 17 799 Euro: (914,51 • y + 1 400) • y; 3. von 17 800 Euro bis 69 878 Euro: (173,10 • z + 2 397) • z + 1 034,87; 4. von 69 879 Euro bis 277 825 Euro: 0,42 • x – 11 135,63; 5. von 277 826 Euro an: 0,45 • x – 19 470,38. Die Größe „y“ ist ein Zehntausendstel des den Grundfreibetrag übersteigenden Teils des auf einen vollen Euro-Betrag abgerundeten zu versteuernden Einkommens. Die Größe „z“ ist ein Zehntausendstel des 17 799 Euro übersteigenden Teils des auf einen vollen Euro-Betrag abgerundeten zu versteuernden Einkommens. Die Größe „x“ ist das auf einen vollen Euro-Betrag abgerundete zu versteuernde Einkommen. Der sich ergebende Steuerbetrag ist auf den nächsten vollen Euro-Betrag abzurunden.

The splitting procedure (Splittingverfahren, § 32a Abs. 5 EStG) applies the same tariff to half of the joint income, and doubles the result.

## Rules

Let $x$ be the basis in whole euros: the taxable income for a single filer, or half of the joint taxable income, rounded down, for a joint filer. In each zone the tax is

$$
T(x) = \left\lfloor a\,t^2 + b\,t + c \right\rfloor, \qquad t = \frac{x - \text{base}}{\text{step}}
$$

with the zone's coefficients $a$, $b$, $c$ from the table. The zone is the last one whose start is at or below $x$. For the two linear zones $t = x$, so $a = 0$. The floor is taken once, at the end of the zone, as the statute requires. Below the basic allowance the tax is $0$.

For a joint filer, the tax is $2\,T(\lfloor z / 2 \rfloor)$ for taxable income $z$.

## Exact form

The implementation uses integers only. With the coefficients scaled by 100 (so $a = a_h / 100$ and so on), and the step $s$:

$$
T(x) = \left\lfloor \frac{a_h\,d^2 + b_h\,d\,s + c_h\,s^2}{100\,s^2} \right\rfloor, \qquad d = x - \text{base}
$$

The progressive zones use $s = 10\,000$, and the linear zones use $s = 1$. Every intermediate value fits in `i128` for any income `Cents` can hold, which the Kani harness `tariff_never_panics` proves for VZ 2026.

## Checks

- Zones join without a jump: the tax changes by at most one euro at every zone start, in every year. The build checks this for every `rules/<year>.toml`.
- The zero-tax zone of VZ 2026 extends a few euros above the basic allowance, because tax rounds down: $T(12\,355) = 0$ and $T(12\,356) = 1$.
- Single filer at 30 000 € in VZ 2026: 4 217 €. Joint filer at 100 000 € in VZ 2026: 21 096 €.
- The coefficients were checked against the BMF withholding guidance (Programmablaufplan, PAP) text of each year. For 2024 to 2026 they were checked against the statute text. For 2014, the statute text of the act to reduce cold progression, as transcribed on buzer.de (an unquoted secondary source), gives the same coefficients. The 2014 PAP is a draft, per its file name.
- The tax at 20 000 €, 40 000 €, 100 000 €, 250 000 € and 300 000 € in each year matches an independent calculation from the same tables. These are in the test `tariff_matches_independent_calculation_for_every_year`.
- For VZ 2026, a comparison outside the repository covered every whole euro from 0 € to 300 000 €, single and joint filers. All 600 007 values matched.

## Coefficients by year

Values are in euros, with decimal points. Each zone's formula is $a\,t^2 + b\,t + c$, and the table lists $a$ and $b$ for the quadratic zones, and the rate and $c$ for the linear zones. Zone 2 starts at the basic allowance plus one euro. In VZ 2006 the top rate was 42 %; the 45 % rate starts in VZ 2007.

| VZ | Basic allowance | Zone 2 from | a | b | Zone 3 from | a | b | c | Zone 4 from | rate | c | Zone 5 from | rate | c |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 2006 | 7 664 € | 7 665 € | 883.74 | 1 500 | 12 740 € | 228.74 | 2 397 | 989 | 52 152 € | 0.42 | −7 914 | — | — | — |
| 2007 | 7 664 € | 7 665 € | 883.74 | 1 500 | 12 740 € | 228.74 | 2 397 | 989 | 52 152 € | 0.42 | −7 914 | 250 001 € | 0.45 | −15 414 |
| 2008 | 7 664 € | 7 665 € | 883.74 | 1 500 | 12 740 € | 228.74 | 2 397 | 989 | 52 152 € | 0.42 | −7 914 | 250 001 € | 0.45 | −15 414 |
| 2009 | 7 834 € | 7 835 € | 939.68 | 1 400 | 13 140 € | 228.74 | 2 397 | 1 007 | 52 552 € | 0.42 | −8 064 | 250 401 € | 0.45 | −15 576 |
| 2010 | 8 004 € | 8 005 € | 912.17 | 1 400 | 13 470 € | 228.74 | 2 397 | 1 038 | 52 882 € | 0.42 | −8 172 | 250 731 € | 0.45 | −15 694 |
| 2011 | 8 004 € | 8 005 € | 912.17 | 1 400 | 13 470 € | 228.74 | 2 397 | 1 038 | 52 882 € | 0.42 | −8 172 | 250 731 € | 0.45 | −15 694 |
| 2012 | 8 004 € | 8 005 € | 912.17 | 1 400 | 13 470 € | 228.74 | 2 397 | 1 038 | 52 882 € | 0.42 | −8 172 | 250 731 € | 0.45 | −15 694 |
| 2013 | 8 130 € | 8 131 € | 933.70 | 1 400 | 13 470 € | 228.74 | 2 397 | 1 014 | 52 882 € | 0.42 | −8 196 | 250 731 € | 0.45 | −15 718 |
| 2014 | 8 354 € | 8 355 € | 974.58 | 1 400 | 13 470 € | 228.74 | 2 397 | 971 | 52 882 € | 0.42 | −8 239 | 250 731 € | 0.45 | −15 761 |
| 2015 | 8 472 € | 8 473 € | 997.60 | 1 400 | 13 470 € | 228.74 | 2 397 | 948.68 | 52 882 € | 0.42 | −8 261.29 | 250 731 € | 0.45 | −15 783.19 |
| 2016 | 8 652 € | 8 653 € | 993.62 | 1 400 | 13 670 € | 225.40 | 2 397 | 952.48 | 53 666 € | 0.42 | −8 394.14 | 254 447 € | 0.45 | −16 027.52 |
| 2017 | 8 820 € | 8 821 € | 1 007.27 | 1 400 | 13 770 € | 223.76 | 2 397 | 939.57 | 54 058 € | 0.42 | −8 475.44 | 256 304 € | 0.45 | −16 164.53 |
| 2018 | 9 000 € | 9 001 € | 997.80 | 1 400 | 13 997 € | 220.13 | 2 397 | 948.49 | 54 950 € | 0.42 | −8 621.75 | 260 533 € | 0.45 | −16 437.70 |
| 2019 | 9 168 € | 9 169 € | 980.14 | 1 400 | 14 255 € | 216.16 | 2 397 | 965.58 | 55 961 € | 0.42 | −8 780.90 | 265 327 € | 0.45 | −16 740.68 |
| 2020 | 9 408 € | 9 409 € | 972.87 | 1 400 | 14 533 € | 212.02 | 2 397 | 972.79 | 57 052 € | 0.42 | −8 963.74 | 270 501 € | 0.45 | −17 078.74 |
| 2021 | 9 744 € | 9 745 € | 995.21 | 1 400 | 14 754 € | 208.85 | 2 397 | 950.96 | 57 919 € | 0.42 | −9 136.63 | 274 613 € | 0.45 | −17 374.99 |
| 2022 | 10 347 € | 10 348 € | 1 088.67 | 1 400 | 14 927 € | 206.43 | 2 397 | 869.32 | 58 597 € | 0.42 | −9 336.45 | 277 826 € | 0.45 | −17 671.20 |
| 2023 | 10 908 € | 10 909 € | 979.18 | 1 400 | 16 000 € | 192.59 | 2 397 | 966.53 | 62 810 € | 0.42 | −9 972.98 | 277 826 € | 0.45 | −18 307.73 |
| 2024 | 11 784 € | 11 785 € | 954.80 | 1 400 | 17 006 € | 181.19 | 2 397 | 991.21 | 66 761 € | 0.42 | −10 636.31 | 277 826 € | 0.45 | −18 971.06 |
| 2025 | 12 096 € | 12 097 € | 932.30 | 1 400 | 17 444 € | 176.64 | 2 397 | 1 015.13 | 68 481 € | 0.42 | −10 911.92 | 277 826 € | 0.45 | −19 246.67 |
| 2026 | 12 348 € | 12 349 € | 914.51 | 1 400 | 17 800 € | 173.10 | 2 397 | 1 034.87 | 69 879 € | 0.42 | −11 135.63 | 277 826 € | 0.45 | −19 470.38 |

## Not yet in fiskus

- Years before 2006, and VZ 2002 to 2005.
- The special cases named in § 32a Abs. 1 Satz 2 EStG, such as the progression reservation.
- Solidarity surcharge (Solidaritätszuschlag) and church tax (Kirchensteuer).
