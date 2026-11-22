import type { Explanation } from '../lib/fiskus';
import { euros, percent } from '../lib/format';
import { Layers, Percent, Receipt, Shield, TrendingUp } from 'lucide-react';
import { useI18n } from '../lib/i18n';
import { Tooltip } from './Tooltip';

export function Result({ result }: { result: Explanation }) {
  const { t } = useI18n();
  const inZeroZone = result.zone === null;
  return (
    <section className="card result" aria-label="Result">
      <p className="eyebrow"><Receipt size={16} /> {t('incomeTax', { year: result.year })}</p>
      <p className="big">{euros(result.income_tax_eur)}</p>
      <dl className="stats">
        <div className="stat">
          <div className="stat-icon"><Percent size={22} /></div>
          <div>
            <dt><Tooltip label={t('averageRate')} text={t('termAverageRate')} german={t('termAverageRateDe')} /></dt>
            <dd>{percent(result.average_rate)}</dd>
          </div>
        </div>
        <div className="stat">
          <div className="stat-icon"><TrendingUp size={22} /></div>
          <div>
            <dt><Tooltip label={t('marginalRate')} text={t('termMarginalRate')} german={t('termMarginalRateDe')} /></dt>
            <dd>{inZeroZone ? '0 %' : percent(result.marginal_rate)}</dd>
          </div>
        </div>
        <div className="stat">
          <div className="stat-icon"><Layers size={22} /></div>
          <div>
            <dt><Tooltip label={t('tariffZone')} text={t('termTariffZone')} german={t('termTariffZoneDe')} /></dt>
            <dd>{inZeroZone ? 'Zero zone (Grundfreibetrag)' : `Zone ${result.zone?.number}`}</dd>
          </div>
        </div>
        <div className="stat">
          <div className="stat-icon"><Shield size={22} /></div>
          <div>
            <dt><Tooltip label={t('basicAllowance')} text={t('termBasicAllowance')} german={t('termBasicAllowanceDe')} /></dt>
            <dd>{euros(result.basic_allowance_eur)}</dd>
          </div>
        </div>
      </dl>
      {result.filing === 'joint' && (
        <p className="note">
          Joint filing (splitting procedure, Splitting): the tariff is applied to half of {euros(result.taxable_income_eur)}, which
          is {euros(result.basis_eur)}, and the result is doubled.
        </p>
      )}
    </section>
  );
}
