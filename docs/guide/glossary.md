{{#include assets/icons/glossary.svg}}

# Glossary

Terms used in the guide, with their German originals. Each term is listed under the first letter of its English name.

**Jump to:** [A](#a) · [B](#b) · [C](#c) · [E](#e) · [F](#f) · [I](#i) · [J](#j) · [M](#m) · [P](#p) · [S](#s) · [T](#t)

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
| Basic allowance | Grundfreibetrag | The zero-tax zone at the start of the tariff, set for each year by [§ 32a Abs. 1 Satz 2 Nr. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html). It is not subtracted from income. Implemented. |
| Basic tariff | Grundtarif | The tariff for single assessment ([§ 26a EStG](https://www.gesetze-im-internet.de/estg/__26a.html)). |

## C

| English | German | Context |
|---|---|---|
| Child allowances | Freibeträge (for children) | Allowances for each child, subtracted from income to reach taxable income. Not yet in fiskus. |
| Church tax | Kirchensteuer | A tax members of a church pay, set by each state. Not yet in fiskus. |

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
| Free threshold | Freigrenze | An amount below which a levy is zero. The solidarity surcharge has one, with a mitigation zone above it. Set for each year in § 3 Abs. 3 SolZG, and in fiskus through `solidarity_free_threshold_eur`. |

## I

| English | German | Context |
|---|---|---|
| Income tax | Einkommensteuer | The tax on the income a person earns in a year. |
| Income Tax Act | Einkommensteuergesetz (EStG) | The statute behind every chapter. Its citations are written in German. |
| Income types | Einkünfte | The seven types of income, such as work, investments, rent and business ([§ 2 Abs. 1 EStG](https://www.gesetze-im-internet.de/estg/__2.html)). Not yet in fiskus. |

## J

| English | German | Context |
|---|---|---|
| Joint assessment | Zusammenveranlagung | Assessment of two spouses or registered partners together ([§ 26b EStG](https://www.gesetze-im-internet.de/estg/__26b.html)). In fiskus, `FilingStatus::Joint`. |

## M

| English | German | Context |
|---|---|---|
| Marginal tax rate | Grenzsteuersatz | The tax on the next euro of taxable income: the derivative of the zone's formula. In fiskus, `Explanation.marginal_rate`. |

## P

| English | German | Context |
|---|---|---|
| Parental allowance | Elterngeld | A benefit for parents, worked out under its own law. It can change the rate of tax on taxable income (see progression reservation). Not yet in fiskus. |
| Parental Allowance Act | Bundeselterngeld- und Elternzeitgesetz (BEEG) | The law that sets the parental allowance. |
| Parliament | Bundestag | Passes changes to the tax laws. |
| Progression reservation | Progressionsvorbehalt | Tax-free benefits, such as parental allowance, raise the rate applied to taxable income. Not yet in fiskus. |
| Withholding guidance | Programmablaufplan (PAP) | The guidance the Federal Ministry of Finance publishes for employers' withholding. It applies the tariff's figures, and fiskus checks its figures against it. |

## S

| English | German | Context |
|---|---|---|
| Single assessment | Einzelveranlagung | Assessment of one person ([§ 26a EStG](https://www.gesetze-im-internet.de/estg/__26a.html)). In fiskus, `FilingStatus::Single`. |
| Solidarity surcharge | Solidaritätszuschlag | A surcharge on income tax, set by the Solidarity Surcharge Act ([SolZG](https://www.gesetze-im-internet.de/solzg_1995/)). In fiskus, `solidarity_surcharge`. |
| Solidarity Surcharge Act | Solidaritätszuschlaggesetz 1995 (SolZG) | The law that sets the solidarity surcharge, its free threshold and its mitigation zone. |
| Special expenses | Sonderausgaben | Expenses the law lets you subtract from income, such as certain insurance and donations. Not yet in fiskus. |
| Splitting tariff | Splittingtarif | The tariff for joint assessment: tax on half the joint income, doubled ([§ 32a Abs. 5 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). |

## T

| English | German | Context |
|---|---|---|
| Tariff | Tarif | The schedule of rates that turns taxable income into tax ([§ 32a EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). |
| Tariff tax | tarifliche Einkommensteuer | The tax the tariff gives for taxable income, before credits. In fiskus, `Assessment.income_tax`. |
| Tariff zone | Tarifzone | One of the progressive zones of the tariff, each with its own formula. In fiskus, `ZoneStep`, returned by `tariff_zones`. |
| Taxable income | zu versteuerndes Einkommen (zvE) | The amount the tariff applies to, rounded down to whole euros ([§ 32a Abs. 1 Satz 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). Input today. |
| Tax office | Finanzamt | The local office that makes the assessment. |
| Total income | Gesamtbetrag der Einkünfte | The seven income types added up, before allowances and special expenses ([§ 2 Abs. 3 EStG](https://www.gesetze-im-internet.de/estg/__2.html)). Not yet in fiskus. |
