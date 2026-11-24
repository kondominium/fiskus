# References

Shadow copies of the sources behind `rules/<year>.toml`. Government sites move or change without notice, so each source is copied here, with its hash, to keep the build reproducible. Each parameter's `reference` field points at its file here.

Source tiers, strongest first:

- **primary**: the official Bundesgesetzblatt (BGBl.) text of the law.
- **official**: a document from the Federal Ministry of Finance (BMF) or the Bundestag that states the amount. The BMF withholding guidance (Programmablaufplan, PAP) is an official implementation of the statute. Its value is confirmed, but the law is not quoted.
- **unquoted secondary**: a third-party transcription of the statute. Kept as a record, superseded where an official document covers the same value.

## Basic allowance sources

| File | Source | Used for | Status | Retrieved from | sha256 |
|---|---|---|---|---|---|
| `2024-bgbl-steuerfortentwicklungsgesetz-I-449.pdf` | Tax Development Act (Steuerfortentwicklungsgesetz, SteFeG), BGBl. 2024 I Nr. 449, Art. 1 and 2 | VZ 2025, 2026 | primary, MD5 `b0dd71ec…` matches the BGBl page | https://www.recht.bund.de/bgbl/1/2024/449/regelungstext.pdf?__blob=publicationFile&v=1 | `0a679eafcaf3a658b84a5ef0a69992539fee7052cc80eca6fe2a04a9e56ddcf3` |
| `2024-bgbl-existenzminimum-I-386.pdf` | Act on the Existenzminimum 2024 (Gesetz zur steuerlichen Freistellung des Existenzminimums 2024), BGBl. 2024 I Nr. 386, Art. 1, read in full | VZ 2024 | primary | https://www.recht.bund.de/bgbl/1/2024/386/regelungstext.pdf?__blob=publicationFile&v=1 | `411b3b7fedbb61a68a1ce0286571c5454f6f700843aed54dcf595c5fac5569b5` |
| `2006-bmf-pap-2006.pdf` | BMF Programmablaufplan 2006 | VZ 2006 | official | https://www.bundesfinanzministerium.de/Content/DE/Downloads/Steuern/Steuerarten/Lohnsteuer/Programmablaufplan/006_PAP_2006_a.pdf | `f168862107f9dab98ed61b115406dd650cbe77b16e733fcc039fe4b517b12e6c` |
| `2007-bmf-pap-2007.pdf` | BMF Programmablaufplan 2007 | VZ 2007 | official | …/010_PAP_2007_a.pdf | `0ebe26c1fd81258dd27fd4c6b01eb1aacd68342215fe81aa192466f420c99c6e` |
| `2008-bmf-pap-2008.pdf` | BMF Programmablaufplan 2008 | VZ 2008 | official | …/011_PAP_2008_a.pdf | `e5e19a91d4c70629379e6b124a3f07c99048b393ad4e6ace15ae72af53af4ff7` |
| `2009-bmf-pap-2009.pdf` | BMF Programmablaufplan 2009 | VZ 2009 | official | …/012_PAP_2009_a.pdf | `8bf6c6127ed75e88cac4e843f00ee2493b0ed2291821ba83a6cb7e0ee8fea660` |
| `2010-bmf-pap-2010.pdf` | BMF Programmablaufplan 2010 | VZ 2010 | official | …/005_c_PAP_2010_Austausch.pdf | `e2923ee323c3111d87065b430e348d0d3fc6a8b28c1397d2a6e591856966a79a` |
| `2011-bmf-pap-dezember-2011.pdf` | BMF Programmablaufplan, December 2011 | VZ 2011 | official | …/003_b_PAP_Dez_2011.pdf | `c31a895e906deb1f221a1e81730c25e6ac21c56106f5802e2e33efe31f0336ea` |
| `2011-bmf-pap-bis-november-2011.pdf` | BMF Programmablaufplan 2011, to November | not used (same value as December) | official | …/002_a_PAP_2011_maschinell.pdf | `fc0da7dab08b60dbf1d62303d369eaf33a88aa385a2cd8993bc69439f607e0be` |
| `2012-bmf-pap-2012.pdf` | BMF Programmablaufplan 2012 | VZ 2012 | official | …/004_b_PAP_2012.pdf | `d899e082605ac8eacae6466843af11e9505279cfb6d4e884be3d396aa04c71e8` |
| `2013-bmf-pap-2013.pdf` | BMF Programmablaufplan 2013, amended | VZ 2013 | official, states "auf 8 130 Euro" | …/2013-02-20-PAP-2013_geaendert.pdf | `c63fa60c31839b4be2accb7b76c74d5a265e7adf2d2bf6e30dcee29deedf0b3c` |
| `2014-bmf-pap-2014.pdf` | BMF Programmablaufplan 2014, **draft** (Entwurf), per the file name | VZ 2014 | official draft, states "auf 8.354 Euro" | …/2013-12-03-entwuerfe-PAP-2014-anlage-1.pdf | `320f43ad3c73afce8acf55983b195144086f4ba1f9def8c6068cd184a8881c2f` |
| `2015-bmf-pap-dezember-2015.pdf` | BMF Programmablaufplan, December 2015 | VZ 2015 | official, states the raise for 2015 | …/2015-09-08-PAP-dezember-2015-anlage-1.pdf | `02ae73aeed6b84836808f3ec7433f9ba70566718a71b34d89ad08b4e7bff6fd2` |
| `2015-bmf-pap-bis-november-2015.pdf` | BMF Programmablaufplan 2015, to November | not used (still 8 354 €) | official | …/2014-11-26-PAP-2015-anlage-1.pdf | `49cfa9d5fe27a8a86ab497d442a19b13cc3c42cb18e0b52252d4e122842af260` |
| `2016-bmf-pap-2016.pdf` | BMF Programmablaufplan 2016 | VZ 2016 | official | …/2015-11-16-PAP-2016-anlage-1.pdf | `ad61fe7ab1d19059bdbcfd0a9cc66d1d2a4103e63bca45bc058055600352c6c4` |
| `2017-bmf-pap-2017.pdf` | BMF Programmablaufplan 2017 | VZ 2017 | official, states "auf 8.820 Euro" | …/2016-11-11-PAP-2017.pdf | `a7040e4134e2116ba2bbb44836e05fdf92b2b58d5bc9052ab0ade29557c26d67` |
| `2018-bmf-pap-2018.pdf` | BMF Programmablaufplan 2018, amended | VZ 2018 | official, states "auf 9.000 Euro" | …/2017-11-23-PAP-2018-anlage-1-geaendert.pdf | `53ec550666128290364ec922eec2736cae1fdfbc97c53a9c056a82c0340debfd` |
| `2019-bmf-pap-2019.pdf` | BMF Programmablaufplan 2019 | VZ 2019 | official, states "auf 9.168 Euro" | …/2018-11-12-PAP-2019-anlage-1.pdf | `a6345b944596a99c52e3570be336e935c318ddb9ae2f6655a0edfb17770d291f` |
| `2020-bmf-pap-2020.pdf` | BMF Programmablaufplan 2020 | VZ 2020 | official, states "auf 9.408 Euro" | …/2019-11-11-PAP-2020-anlage-1.pdf | `5c912f6ef3a30472d2ae401677673ceac38f9781213d023a036346941ff9075d` |
| `2021-bmf-pap-2021.pdf` | BMF Programmablaufplan 2021 | VZ 2021 | official, states "auf 9.744 Euro" | …/2020-11-09-PAP-2021-anlage-1.pdf | `d82653ced872464b79059aa2ca1801643aa7615a00bac4ec0f7b1c7029c2ccdd` |
| `2022-bmf-pap-2022.pdf` | BMF Programmablaufplan 2022, amended | VZ 2022 | official, states "auf 10.347 Euro" | …/2022-05-20-geaenderte-PAP-2022-anwendung-ab-01-06-2022-anlage-1.pdf | `74e8660dc87391eda0af96587bf682f138aadf7ebcfc7e2273bc116ef6838bc0` |
| `2023-bmf-pap-ab-juli-2023.pdf` | BMF Programmablaufplan 2023, from July | VZ 2023 | official, states "auf 10.908 Euro" | …/2023-06-09-geaenderte-PAP-2023-anwendung-ab-dem-1-juli-2023-anlage-1.pdf | `1b99ae598dc20b41997984f6909623b44598c7a7f0dba1bb7e5e4719f50203c4` |
| `2023-bmf-pap-bis-juni-2023.pdf` | BMF Programmablaufplan 2023, to June | not used (same value) | official | …/2023-02-13-geaenderte-PAP-2023-anwendung-ab-dem-1-april-2023-anlage-1.pdf | `bb34f6719ecd210653b89a173d542ecbabbdf59e3890c490673fc5b0c60433c5` |
| `2024-bmf-pap-letter-2024.pdf` | BMF letter of 23 February 2024 on the amended PAP 2024. It contains no PAP itself, so the 2024 value is taken from the statute | not used for values | official letter | …/2024-02-23-geaenderte-PAP-2024-anwendung-ab-dem-1-april-2024-bmf-schreiben.pdf | `e7aebd4686a0eda70e7709a5de56a8fa9afb4e48382479fd216117fc3fa2d2ce` |
| `2025-bmf-pap-2025.pdf` | BMF Programmablaufplan 2025, amended | VZ 2025 check, states "auf 12.096 Euro" | official | …/2025-01-22-geaenderte-PAP-2025-anlage-1.pdf | `092de325b1e3258cff16a617b1963f923a82e98952e53c345ba7403ddbd45a53` |
| `2026-bmf-pap-2026.pdf` | BMF Programmablaufplan 2026, `GFB = 12348` | VZ 2026 check | official | …/2025-11-12-PAP-2026-anlage-1.pdf | `5da38acd544868ddcb0bc7f43030bd7d6ee9aa455dff0f42ad59732634e33a48` |

The PAP URLs share the base `https://www.bundesfinanzministerium.de/Content/DE/Downloads/Steuern/Steuerarten/Lohnsteuer/Programmablaufplan/`, abbreviated `…/` above. Each PAP is also listed by the [Lohnsteuer repository](https://github.com/MarcelLehmann/Lohnsteuer) (Apache-2.0, a generator built from these PAPs). We used the BMF files directly and copied no code from it.

## Bundestag documents

| File | Source | Used for | Status | sha256 |
|---|---|---|---|---|
| `2017-bt-drs-existenzminimumbericht-18-10220.pdf` | 11th report on the subsistence minimum, BT-Drs. 18/10220 | VZ 2017, 2018 | official report | `d1f1eeb41935a98bcbc343ce33ff0edeecc2dc70718a5c07449570fdc3e0133f` |
| `2019-bt-drs-existenzminimumbericht-19-05400.pdf` | 12th report, BT-Drs. 19/5400 | VZ 2018 (fn. 6), 2019, 2020 | official report | `75147c0e288e45c84228f13243c3ae98e090041db3d3f75b7b8d0fba4fd4db39` |
| `2022-bt-drs-existenzminimumbericht-19-22800.pdf` | 13th report, BT-Drs. 19/22800 | VZ 2020, 2022 context | official report | `a8342cccf9fd20e66dbaaaba54f49e8836aed4de6e481bae4b005ee0d7969367` |
| `2024-bt-drs-existenzminimumbericht-20-04443.pdf` | 14th report, BT-Drs. 20/4443 | VZ 2024 context | official report | `31038bb446f4fd270880c103747e8413724b47c55623d058e0820aa67cc61b5f` |
| `2022-bt-drs-inflationsausgleichsgesetz-entwurf-20-3496.pdf` | Inflation Compensation Act draft (Inflationsausgleichsgesetz, InflAusG), BT-Drs. 20/3496, with the reasons for 2022 and 2023 | VZ 2022, 2023 context | draft | `d4cb500f899290599ca676edd0fd738b21a96277e4ed45e6fd54c18c973decd7` |
| `2003-bt-drs-existenzminimum-14-7765.pdf` | Report on the subsistence minimum for 2003, BT-Drs. 14/7765, footnote 6 cites the Tax Cut Act (BGBl. I 2000 S. 1433) | VZ 2003 (not in fiskus yet) | official report, gives 7 426 € | `12924029bc8bcdddfacf33efafe3366a72cb041576ccbdc3cfc1627adfd0582e` |
| `2000-bt-drs-steuersenkungsgesetz-entwurf-14-3074.pdf` | Tax Cut Act draft, BT-Drs. 14/3074 | context | draft | `594a588c13e66848e9a0cd32de9796db690270ea1eb5d6b0482e8978863b1b17` |
| `2004-bt-drs-haushaltsbegleitgesetz-entwurf-15-1502.pdf` | Budget Accompanying Act 2004 draft, BT-Drs. 15/1502 | VZ 2004, 2005 | draft, states "bis 7 664 Euro (Grundfreibetrag)" | `c009f601079bbadcf085e994a4aebdf7e3f71a9da620215707b2ba5c36ada75e` |
| `2005-bt-drs-existenzminimum-15-2462.pdf` | Report on the subsistence minimum for 2005, BT-Drs. 15/2462 | VZ 2005 context | official report | `3d1f5ee5a107b42d662d2a359f18ca47c171239e593b9ce006b361cbdd19c3fe` |
| `2008-bt-drs-existenzminimum-16-3265.pdf` | Report on the subsistence minimum for 2008, BT-Drs. 16/3265 | VZ 2008 context | official report | `6632c5df53b225f95d51f659a0829841e64784d536f3c7f7c6d9f5fd441e9ca5` |
| `2009-bt-drs-stabsig-entwurf-16-11740.pdf` | StabSiG draft, BT-Drs. 16/11740 | VZ 2009 context | draft | `acfc28eb8f18b7a72b7b281b5d44cc55dd3b48defe00679e7c328887ae346dc9` |
| `2009-bundestag-stabsig-xvi-0445.pdf` | Bundestag legislative documentation on the StabSiG | VZ 2009 context | official | `1e1cf51766b6a58178c3a1a9bdb0738d1de2b49aafda520c19ee3df449a27de4` |
| `2015-bt-drs-kigaanhg-entwurf-18-4649.pdf` | KiGAnhG draft, BT-Drs. 18/4649 | VZ 2015 context | draft | `67391a46e517b1f13cf8a45040b88aa70ae3387d91340fc4042b6a86da3adfef` |

## Unquoted secondary: superseded for values

These buzer.de transcriptions of the statute are kept as a record. For VZ 2009 to 2016 the BMF PAPs above now confirm the values.

| File | Source | Status | sha256 |
|---|---|---|---|
| `secondary-buzer-gesetz-8650-stabsig-2009.txt` | StabSiG, transcribed on buzer.de | unquoted secondary | `d76be1e03d356f0296e412e3ebc3b90ed0deef29d1ce86184365e1d09839b1ce` |
| `secondary-buzer-gesetz-10510-abbau-kalte-progression-2013.txt` | Act to Reduce Cold Progression (Gesetz zum Abbau der kalten Progression), BGBl. 2013 I S. 283, transcribed on buzer.de | unquoted secondary | `b3f69fcf7e378e19465de551cad5fca4b7cd83e75c8971228c599d63793eca2b` |
| `secondary-buzer-gesetz-11668-kigaanhg-2015.txt` | KiGAnhG, BGBl. 2015 I S. 1202, transcribed on buzer.de | unquoted secondary | `68a2a8de1910fb79fa167d5e6832f597e34c205190f02296a3f30835f7d67e59` |
| `secondary-buzer-gesetz-9913-stvereinfg-2011.txt` | Tax Simplification Act 2011 (Steuervereinfachungsgesetz 2011), transcribed on buzer.de; no basic allowance text in it | context only | `ca48bf3143434d310bff7b963bb24fa1f85d05d6b8348a897c84cad6cdef589f` |

## Not yet in fiskus

- **VZ 2002 to 2004.** Not archived. The Lohnsteuer repository lists PAPs from 2006 on, so the sources for these years still need a search. VZ 2005 is now in fiskus, sourced from the Haushaltsbegleitgesetz 2004 draft (BT-Drs. 15/1502).
- **Dates and sources to check later.** The 2014 PAP is a draft. The 2006 to 2008 setting laws are not traced yet, although their values are confirmed.
- **bgbl.de and recht.bund.de.** recht.bund.de serves issues from 2023 on, such as BGBl. 2024 I Nr. 386 above. It returned 404 for BGBl. 2022 I Nr. 49 and Nr. 17, two issues that dejure.org lists, so pre-2023 issues are not there. bgbl.de serves a JavaScript page for every URL form tried.

## Tariff sources (`rules/<year>.toml`)

- **BMF withholding guidance (PAP), VZ 2006 to 2023.** The coefficients were checked against each year's PAP text in `docs/references`. The Lohnsteuer repository (Apache-2.0) mirrors the BMF's XML and generated Java for each year. We read the tariff method from that Java and copied no code.
- **Statute, VZ 2024 to 2026.** BGBl. 2024 I Nr. 386 and Nr. 449, in `docs/references`.
- **Not stored:** the repository's XML and Java files. Their origin at the BMF is not yet verified, because the bmf-steuerrechner.de site returned HTTP 503 during this work.

## Lead sources (not stored)

- dejure.org lists every law that amended § 32a EStG, with its BGBl. page: https://dejure.org/gesetze/EStG/32a.html. It is an index for finding the setting laws, not a statute.
- The [Lohnsteuer repository](https://github.com/MarcelLehmann/Lohnsteuer) indexes the BMF PAPs from 2006 to 2026 with their URLs.

## Adding a source

1. Download the file with `curl -sSL`, check it with `file`, and record its sha256 with `shasum -a 256`.
2. Name it `<year>-bgbl-<slug>-I-<nr>.pdf` for a primary source, `<year>-bmf-pap-<qualifier>-<year>.pdf` for a BMF PAP, or `<year>-bt-drs-<slug>-<wahlperiode>-<nr>.pdf` for a Bundestag document. Drop the `secondary` marker.
3. Add a row here and point the `reference` field of the parameter file at it.
