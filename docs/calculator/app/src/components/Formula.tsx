import type { Explanation } from '../lib/fiskus';
import { coefficient, euros, grouped } from '../lib/format';
import { Calculator } from 'lucide-react';
import { useI18n } from '../lib/i18n';

// Shows the zone formula with the numbers of this income, so the tax can be checked by hand.
export function Formula({ result }: { result: Explanation }) {
  const { t } = useI18n();
  const zone = result.zone;
  if (!zone) {
    return (
      <div className="formula" aria-label="Formula">
        <h2><Calculator size={20} /> {t('howTaxWorked')}</h2>
        <p>
          The basis ({euros(result.basis_eur)}) is within the basic allowance ({euros(result.basic_allowance_eur)}),
          so the tax is 0 €.
        </p>
      </div>
    );
  }

  const tValue = (result.basis_eur - zone.base_eur) / zone.step_eur;
  const quadratic = zone.quadratic;
  const linear = zone.linear;
  const constant = zone.constant;
  const isQuadratic = zone.quadratic !== 0;

  return (
    <div className="formula" aria-label="Formula">
      <h2><Calculator size={20} /> {t('howTaxWorked')}</h2>
      <p className="muted">
        Zone {zone.number}, from {euros(zone.from_eur)}. Basis {euros(result.basis_eur)}.
      </p>
      {isQuadratic ? (
        <>
          <p>
            <span className="math">
              t = (x − {grouped(zone.base_eur)}) / {grouped(zone.step_eur)} = {tValue.toFixed(4)}
            </span>
          </p>
          <p>
            <span className="math">
              tax = {coefficient(quadratic)} · t² + {coefficient(linear)} · t
              {constant !== 0 && ` + ${coefficient(constant)}`}
            </span>
          </p>
        </>
      ) : (
        <p>
          <span className="math">
            tax = {coefficient(linear)} · x {constant < 0 ? '−' : '+'} {coefficient(Math.abs(constant))}
          </span>
        </p>
      )}
      <p>
        Unrounded: <span className="math">{euros(result.unrounded_basis_tax, 2)}</span>. Rounded
        down to whole euros: <strong>{euros(result.basis_tax_eur)}</strong>
        {result.filing === 'joint' && `, doubled to ${euros(result.income_tax_eur)}`}.
      </p>
    </div>
  );
}
