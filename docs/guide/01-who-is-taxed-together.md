{{#include assets/icons/who.svg}}

# Who is taxed together

The first input is the filing unit (Veranlagungseinheit): the people whose income is assessed in one return. **Your personal data, such as your marital status, decides which filing status applies.**

- Single assessment (Einzelveranlagung, [§ 26a EStG](https://www.gesetze-im-internet.de/estg/__26a.html)): one person. The basic tariff (Grundtarif) applies.
- Joint assessment (Zusammenveranlagung, [§ 26b EStG](https://www.gesetze-im-internet.de/estg/__26b.html)): two spouses or registered partners assessed together. The splitting tariff (Splittingtarif) applies. The [basic allowance](04-basic-allowance.md) page shows what that changes.

Spouses who meet the conditions of [§ 26 Abs. 1 EStG](https://www.gesetze-im-internet.de/estg/__26.html) can choose between the two.

In fiskus, a filing unit is a `FilingUnit` with a `FilingStatus` of `Single` or `Joint`. Other personal data, such as children, is not yet in fiskus.
