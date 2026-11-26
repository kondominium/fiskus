# Table steps (Tabellenstufen), § 32a Abs. 2 EStG

## In plain words

For VZ 2002 and VZ 2003, the tax did not use the taxable income itself. The income was first moved to the middle of a block of 36 €, so every income in the same block pays the same tax. The step applies to VZ 2003 because that year keeps the 2002 tariff. Earlier formulas had a similar step in DM, which fiskus does not implement.

## Statute

§ 32a Abs. 2 EStG in the 2002 text, set by the Steuerentlastungsgesetz 1999/2000/2002 (BGBl. 1999 I S. 402). The text reads:

> Das zu versteuernde Einkommen ist auf den nächsten durch 36 ohne Rest teilbaren vollen Euro-Betrag abzurunden, wenn es nicht bereits durch 36 ohne Rest teilbar ist, und um 18 Euro zu erhöhen.

Source: the archived transcription, [`2002-estg-32a-transcription-parmentier.txt`](../references/2002-estg-32a-transcription-parmentier.txt). The enacted text is not archived yet. The BMF tariff history, section 15, gives the same rule for "2002 - 2003" (archived as [`2025-bmf-tarifhistorie-steuerrechner.pdf`](../references/2025-bmf-tarifhistorie-steuerrechner.pdf)).

The sentence reads two ways. fiskus applies the +18 € to every basis, not only to bases that are not multiples of 36. The BMF calculator decides this. The other reading gives 9 of 500 mismatches on the 2002 option, and the reading used here gives none.

## Formula

For a whole-euro basis `x` (the taxable income for single filers, or half of it for joint filers, rounded down):

$$
x' = 36 \left\lfloor \frac{x}{36} \right\rfloor + 18
$$

The zones use `x'` in place of `x`. The zero zone uses `x` itself: a basis up to 7 235 € pays nothing, whatever its block.

For VZ 2003 the zone formulas are the 2002 tariff. With `x'`:

- zone 2, from 7 236 €: `t = (x' − 7 200) / 10 000`, tax `(768,85 · t + 1 990) · t`
- zone 3, from 9 252 €: `t = (x' − 9 216) / 10 000`, tax `(278,65 · t + 2 300) · t + 432`
- zone 4, from 55 008 €: tax `0,485 · x' − 9 872`

Zone 4 has a slope of 48,5 hundredths. fiskus stores it with `step_eur = 10` and `linear_h = 485`, so `t = x' / 10` gives the slope in whole hundredths.

The zone is chosen on `x'`. Choosing it on `x` gives the same tax at every whole euro from 0 to 200 000 €, so either reading is correct for the statute's figures.

## Joint filers

The halving comes first, and the table step applies to the half. For example, a joint income of 14 472 € gives a half of 7 236 €, `x' = 7 254`, and a tax of 10 € on the half. The joint tax is 20 €.

## Implementation

- `rules/2003.toml` has the `[table_step]` table, with `width_eur = 36` and `added_eur = 18`, and its citation.
- `build.rs` checks each table step, and checks the zones on the stepped basis. Every zone must start at a block edge. The tax may move by one block at a zone start, where the basis moves by a whole block.
- `src/tariff.rs` applies the step in `table_basis`, after the zero-zone check. `Explanation::tariff_basis_eur` reports the stepped basis.

## Evidence

- Unit test `year_2003_rounds_the_basis_to_its_block_before_the_zones` pins the boundary values, taken from the reading above.
- Property `tax_is_flat_inside_each_table_block` checks that the tax is constant inside each block, and rises by at most one euro per euro of block.
- Kani proof `table_step_keeps_the_basis_within_its_block_and_monotone` checks that the stepped basis stays within its block, and never decreases.
- Conformance: the BMF option 2002 (2002 to 2003) gives 500 of 500 matches with `make test-conformance YEARS=2002`.

## Not yet in fiskus

- VZ 2002 has no rules file. Its BMF option maps to VZ 2003, which has the same tariff.
- The enacted texts of BGBl. 1999 I S. 402 and BGBl. 2002 I S. 3651 are not archived. The citations rest on the transcription and the bill, BT-Drs. 14/9894.
- No later year has a table step. From VZ 2004 the statute rounds the taxable income to whole euros only (BMF tariff history, section (e) of the rounding notes).
- The DM steps of the formulas for 1981 to 2001 are not in fiskus.
