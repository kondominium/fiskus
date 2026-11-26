{{#include assets/icons/glossary.svg}}

# Glossary

Terms used in the guide, with their German originals. Each term is listed under the first letter of its English name.

**Jump to:** [A](#a) · [B](#b) · [C](#c) · [E](#e) · [F](#f) · [I](#i) · [J](#j) · [M](#m) · [P](#p) · [R](#r) · [S](#s) · [T](#t) · [W](#w) · [Z](#z)

## A

| English | German | Context |
|---|---|---|
| Allowance | Freibetrag | An amount that is exempt from tax or is subtracted before tax. The kinds are listed under basic allowance and child allowances. |
| Assessment | Veranlagung | The process in which the tax office works out the tax for a filing unit. |
| Assessment period | Veranlagungszeitraum (VZ) | The calendar year a tax is assessed for. In fiskus, `TaxYear`. |
| Average tax rate | Durchschnittssteuersatz | The income tax divided by taxable income: the share of income that goes to tax. Also called the effective tax rate. In fiskus, `Explanation.average_rate`. |

## B

| English | German | Context |
|---|---|---|
| Basic allowance | Grundfreibetrag | The zero zone at the start of the tariff, set for each year by [§ 32a Abs. 1 Satz 2 Nr. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html). It is not subtracted from income. Implemented. |
| Basic tariff | Grundtarif | The tariff that applies to one person's own taxable income, with no splitting ([§ 32a EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). |

## C

| English | German | Context |
|---|---|---|
| Child allowances | Freibeträge (for children) | Allowances for each child, subtracted from income to reach taxable income. Not yet in fiskus. |
| Church tax | Kirchensteuer | A tax members of a church pay, set by each state. Not yet in fiskus. |
| Complementary levy | Ergänzungsabgabe | A levy added on top of an existing tax, with its revenue for the federal government alone. The solidarity surcharge is one. |

## E

| English | German | Context |
|---|---|---|
| Extraordinary burdens | außergewöhnliche Belastungen | Costs beyond the ordinary, such as long illness, that the law lets you subtract from income. Not yet in fiskus. |

## F

| English | German | Context |
|---|---|---|
| Federal Law Gazette | Bundesgesetzblatt (BGBl.) | Where laws are published once they are passed. |
| Federal Ministry of Finance | Bundesministerium der Finanzen (BMF) | Administers the Income Tax Act and publishes guidance. It does not write or endorse fiskus. |
| Filing status | Veranlagungsform | Single or joint assessment ([§ 26 EStG](https://www.gesetze-im-internet.de/estg/__26.html)). In fiskus, `FilingStatus`. |
| Filing unit | Veranlagungseinheit | One person, or two spouses assessed together. In fiskus, `FilingUnit`. |
| Flood Victims' Solidarity Act | Flutopfersolidaritätsgesetz | A 2002 act that funded flood relief. It did not use the solidarity surcharge. |
| Free threshold | Freigrenze | An amount below which a levy is zero. The solidarity surcharge has one, with a mitigation zone above it. Set for each year in § 3 Abs. 3 SolZG, and in fiskus through `solidarity_free_threshold_eur`. |

## I

| English | German | Context |
|---|---|---|
| Income tax | Einkommensteuer | The tax on the income a person earns in a year. |
| Income Tax Act | Einkommensteuergesetz (EStG) | The statute behind every chapter. Its citations are written in German. |
| Income types | Einkünfte | The seven types of income, such as work, investments, rent and business ([§ 2 Abs. 1 EStG](https://www.gesetze-im-internet.de/estg/__2.html)). Not yet in fiskus. |
| Inflation Compensation Act | Inflationsausgleichsgesetz | A 2022 act that set the basic allowance for 2023 ([§ 32a Abs. 1 Satz 2 Nr. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). |

## J

| English | German | Context |
|---|---|---|
| Joint assessment | Zusammenveranlagung | Assessment of two spouses or registered partners together ([§ 26b EStG](https://www.gesetze-im-internet.de/estg/__26b.html)). In fiskus, `FilingStatus::Joint`. |
| Joint filers | — | Two spouses taxed as one unit, on their combined taxable income, under joint assessment. No statute uses a single German term for this. |

## M

| English | German | Context |
|---|---|---|
| Marginal tax rate | Grenzsteuersatz | The tax on the next euro of taxable income: the derivative of the zone's formula. In fiskus, `Explanation.marginal_rate`. |
| Mitigation zone | Milderungszone | The band of the solidarity surcharge between the free threshold and the full 5.5 % rate. There the surcharge is capped at 11.9 % of the amount above the threshold. |

## P

| English | German | Context |
|---|---|---|
| Parental allowance | Elterngeld | A benefit for parents, worked out under its own law. It can change the rate of tax on taxable income (see progression reservation). Not yet in fiskus. |
| Parental Allowance Act | Bundeselterngeld- und Elternzeitgesetz (BEEG) | The law that sets the parental allowance. |
| Parliament | Bundestag | Passes changes to the tax laws. |
| Progression reservation | Progressionsvorbehalt | Tax-free benefits, such as parental allowance, raise the rate applied to taxable income. Not yet in fiskus. |
| Withholding guidance | Programmablaufplan (PAP) | The guidance the Federal Ministry of Finance publishes for employers' withholding. It applies the tariff's figures, and fiskus checks its figures against it. |

## R

| English | German | Context |
|---|---|---|
| Rollback of the surcharge | Rückführung des Solidaritätszuschlags | The reduction of the solidarity surcharge. It began with the 2021 relief. |
| Rounding down | Abrundung | The tax is computed on taxable income in whole euros. Cents are cut off, never rounded up ([§ 32a Abs. 1 Satz 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). |

## S

| English | German | Context |
|---|---|---|
| Single assessment | Einzelveranlagung | Assessment of one person on their own income, with no splitting. In fiskus, `FilingStatus::Single`. |
| Single filer | — | One person taxed alone, on their own taxable income, under single assessment. No statute uses a single German term for this. In fiskus, `FilingStatus::Single`. |
| Solidarity Pact II | Solidarpakt II | The federal support for the eastern states from 2005 to 2019. Historically, the surcharge's revenue was tied to it. |
| Solidarity surcharge | Solidaritätszuschlag | A surcharge on income tax, set by the Solidarity Surcharge Act ([SolZG](https://www.gesetze-im-internet.de/solzg_1995/)). In fiskus, `solidarity_surcharge`. |
| Solidarity Surcharge Act | Solidaritätszuschlaggesetz 1995 (SolZG) | The law that sets the solidarity surcharge, its free threshold and its mitigation zone. |
| Special expenses | Sonderausgaben | Expenses the law lets you subtract from income, such as certain insurance and donations. Not yet in fiskus. |
| Splitting tariff | Splittingtarif | The tariff for joint assessment: tax on half the joint income, doubled ([§ 32a Abs. 5 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). |
| Subsistence minimum | Existenzminimum | The amount a person needs to live on. The law leaves it untaxed, and the basic allowance is set from it. |
| Subsistence minimum report | Existenzminimumbericht | The government's regular report on the subsistence minimum. The law is changed to match it. |

## T

| English | German | Context |
|---|---|---|
| Tariff | Tarif | The schedule of rates that turns taxable income into tax ([§ 32a EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). |
| Tariff tax | tarifliche Einkommensteuer | The tax the tariff gives for taxable income, before credits. In fiskus, `Assessment.income_tax`. |
| Tariff zone | Tarifzone | One of the zones of the tariff above the zero zone, each with its own formula. In fiskus, `ZoneStep`, returned by `tariff_zones`. |
| Taxable income | zu versteuerndes Einkommen (zvE) | The amount the tariff applies to, rounded down to whole euros ([§ 32a Abs. 1 Satz 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). Input today. |
| Tax Development Act | Steuerfortentwicklungsgesetz (SteFeG) | The 2024 act that sets the solidarity surcharge thresholds for 2024, 2025 and 2026. |
| Tax rate | Steuersatz | The share of income that tax takes. The average rate is the tax divided by the income. The marginal rate is the tax on the next euro. |
| Tax office | Finanzamt | The local office that makes the assessment. |
| Tax reduction | Steuervergünstigung | A rule that lowers the income that is taxed or the tax due, to reward a choice the state wants to encourage, such as energy-efficient renovation of a home. |
| Total income | Gesamtbetrag der Einkünfte | The seven income types added up, before allowances and special expenses ([§ 2 Abs. 3 EStG](https://www.gesetze-im-internet.de/estg/__2.html)). Not yet in fiskus. |

## W

| English | German | Context |
|---|---|---|
| Work-related expenses | Werbungskosten | Costs of earning the income, such as the travel to work, which are subtracted from the income from work. |

## Z

| English | German | Context |
|---|---|---|
| Zero zone | — | The first zone of the tariff, where the tax is zero. It runs up to and including the basic allowance. No statute uses a single German term for it. |
| Zone rate | — | The marginal rate at the first euro of a tariff zone. No statute uses a single German term for it. |
