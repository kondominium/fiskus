{{#include assets/icons/glossary.svg}}

# Glossary

Terms used in the guide, with their German originals. Each term is listed under the first letter of its English name.

**Jump to:** [A](#a) · [B](#b) · [C](#c) · [E](#e) · [F](#f) · [I](#i) · [J](#j) · [M](#m) · [P](#p) · [S](#s) · [T](#t) · [W](#w)

## A

| English | German | Context |
|---|---|---|
| Allowance | Freibetrag | An amount that is exempt from tax or is subtracted before tax. The kinds are listed under basic allowance and child allowances. |
| Assessment | Veranlagung | The process in which the tax office works out the tax for a filing unit. |
| Assessment period | Veranlagungszeitraum (VZ) | The calendar year a tax is assessed for. In fiskus, `TaxYear`. |
| Average rate | Durchschnittsbelastung | Income tax divided by taxable income, as a fraction of one euro. Implemented. |

## B

| English | German | Context |
|---|---|---|
| Basic allowance | Grundfreibetrag | The zero-tax zone at the start of the tariff: 12 348 € for single filers ([§ 32a Abs. 1 Satz 2 Nr. 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). It is not subtracted from income. Implemented. |
| Basic tariff | Grundtarif | The tariff for single assessment ([§ 26a EStG](https://www.gesetze-im-internet.de/estg/__26a.html)). |

## C

| English | German | Context |
|---|---|---|
| Child allowances | Freibeträge (for children) | Allowances for each child, subtracted from income to reach taxable income. Not yet in fiskus. |
| Church tax | Kirchensteuer | A tax on members of a religious community, charged on top of income tax. Not yet in fiskus. |
| Cold progression | kalte Progression | The extra tax from inflation alone: wages rise with prices, move into higher brackets and pay more tax, though real income has not risen. Not yet in fiskus. |

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

## I

| English | German | Context |
|---|---|---|
| Income tax | Einkommensteuer | The tax on the income a person earns in a year. |
| Income Tax Act | Einkommensteuergesetz (EStG) | The statute behind every chapter. Its citations are written in German. |
| Income types | Einkünfte | The seven types of income, such as work, investments, rent and business ([§ 2 Abs. 1 EStG](https://www.gesetze-im-internet.de/estg/__2.html)). Not yet in fiskus. |
| Inflation | Inflation | The general rise in prices over time. Changes to the basic allowance and the tariff thresholds have responded to it since 2022. |

## J

| English | German | Context |
|---|---|---|
| Joint assessment | Zusammenveranlagung | Assessment of two spouses or registered partners together ([§ 26b EStG](https://www.gesetze-im-internet.de/estg/__26b.html)). In fiskus, `FilingStatus::Joint`. |

## M

| English | German | Context |
|---|---|---|
| Marginal rate | Grenzbelastung | The tax on the next euro of income, from the derivative of the zone's formula. Implemented. |

## P

| English | German | Context |
|---|---|---|
| Parental allowance | Elterngeld | A benefit for parents, worked out under its own law. It can change the rate of tax on taxable income (see progression reservation). Not yet in fiskus. |
| Parental Allowance Act | Bundeselterngeld- und Elternzeitgesetz (BEEG) | The law that sets the parental allowance. |
| Parliament | Bundestag | Passes changes to the tax laws. |
| Progression reservation | Progressionsvorbehalt | Tax-free benefits, such as parental allowance, raise the rate applied to taxable income. Not yet in fiskus. |

## S

| English | German | Context |
|---|---|---|
| Single assessment | Einzelveranlagung | Assessment of one person ([§ 26a EStG](https://www.gesetze-im-internet.de/estg/__26a.html)). In fiskus, `FilingStatus::Single`. |
| Solidarity surcharge | Solidaritätszuschlag | A surcharge on income tax. Not yet in fiskus. |
| Special expenses | Sonderausgaben | Expenses the law lets you subtract from income, such as certain insurance and donations. Not yet in fiskus. |
| Splitting tariff | Splittingtarif | The tariff for joint assessment: tax on half the joint income, doubled ([§ 32a Abs. 5 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). |
| Subsistence minimum | Existenzminimum | The income a person needs for basic living costs, which the tax system must leave untouched. The government estimates it in reports, and the basic allowance is set to match it. |

## T

| English | German | Context |
|---|---|---|
| Tariff | Tarif | The schedule of rates that turns taxable income into tax ([§ 32a EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). |
| Tariff tax | tarifliche Einkommensteuer | The tax the tariff gives for taxable income, before credits. In fiskus, `Assessment.income_tax`. |
| Tax rate | Steuersatz | A rate at which income is taxed. fiskus reports the average rate and the marginal rate. Implemented. |
| Taxable income | zu versteuerndes Einkommen (zvE) | The amount the tariff applies to, rounded down to whole euros ([§ 32a Abs. 1 Satz 1 EStG](https://www.gesetze-im-internet.de/estg/__32a.html)). Input today. |
| Tax office | Finanzamt | The local office that makes the assessment. |

## W

| English | German | Context |
|---|---|---|
| Withholding program flowchart | Programmablaufplan (PAP) | The Federal Ministry of Finance's method for the payroll tax (Lohnsteuer) an employer withholds. fiskus checks its tariff against it, but withholding is not an assessment. Official guidance, not the statute. |
