# Solidarity surcharge (Solidaritätszuschlag), SolZG

The solidarity surcharge is a charge on the income tax. It is zero up to a free threshold, and above it the surcharge rises with a mitigation cap. The statute is the Solidarity Surcharge Act (Solidaritätszuschlaggesetz 1995, SolZG).

## Rule

Let `S` be the income tax of the filing unit in cents, and `F` the free threshold of the year and filing status, from § 3 Abs. 3 SolZG. The surcharge is

$$
\text{surcharge}(S, F) = \min\!\left(\left\lfloor \frac{11}{200}\,S \right\rfloor,\; \left\lfloor \frac{119}{1000}\,(S - F) \right\rfloor\right), \qquad \text{and } 0 \text{ for } S \le F
$$

The first term is 5.5 % of the income tax (§ 4 SolZG). The second term is the mitigation cap, 11.9 % of the amount above the threshold (§ 4 SolZG). The surcharge is never negative. Fractions of a cent are dropped.

The threshold is per year and per filing status. The joint threshold applies to joint filing (§ 3 Abs. 3 Satz 1 Nr. 1 SolZG, for the cases of § 32a Abs. 5 and 6 EStG), and the single one to every other case (Nr. 2).

## Years

The thresholds are in `rules/<year>.toml` under `[solidarity]`, with the citation of the statute that sets them. The statute text of each figure is in `docs/references`. A year without a threshold has no surcharge: `solidarity_surcharge` returns `None` for it.

## What the tests check

- The surcharge is zero at and below the threshold, for both filings, in every year that has one.
- It never decreases as the income tax rises, and it is never negative.
- The mitigation cap holds: the surcharge is at most 11.9 % of the amount above the threshold.
- A year without a threshold gives `None`.

The rates and the threshold figures are checked against the statute text, not against a calculator. Conformance with the BMF calculator is planned work.
