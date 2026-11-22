import { useState } from 'react';
import type { Filing, YearInfo } from '../lib/fiskus';
import { grouped } from '../lib/format';
import { Banknote, Calendar, SlidersHorizontal, User, Users } from 'lucide-react';
import { useI18n } from '../lib/i18n';

interface Props {
  years: YearInfo[];
  year: number;
  onYear: (year: number) => void;
  filing: Filing;
  onFiling: (filing: Filing) => void;
  income: number;
  onIncome: (income: number) => void;
}

export function Controls({ years, year, onYear, filing, onFiling, income, onIncome }: Props) {
  const { t } = useI18n();
  const [filingOpen, setFilingOpen] = useState(false);
  return (
    <section className="card controls" aria-label="Inputs">
      <h2><SlidersHorizontal size={20} /> {t('inputs')}</h2>
      <div className="controls-row">
        <label className="field">
          <span className="field-label"><Calendar size={16} /> {t('assessmentPeriod')}</span>
          <select value={year} onChange={(event) => onYear(Number(event.target.value))}>
            {years.map((entry) => (
              <option key={entry.year} value={entry.year}>
                {entry.year}
              </option>
            ))}
          </select>
        </label>

        <div className="field">
          <span className="field-label"><Users size={16} /> {t('filing')}</span>
          <div className="filing-dropdown">
            <button type="button" className="filing-trigger" onClick={() => setFilingOpen((v) => !v)}>
              {filing === 'single' ? <User size={16} /> : <Users size={16} />}
              <span>{filing === 'single' ? t('single') : t('joint')}</span>
            </button>
            {filingOpen && (
              <div className="filing-menu">
                <button type="button" className={filing === 'single' ? 'active' : ''} onClick={() => { onFiling('single'); setFilingOpen(false); }}>
                  <User size={16} /> {t('single')}
                </button>
                <button type="button" className={filing === 'joint' ? 'active' : ''} onClick={() => { onFiling('joint'); setFilingOpen(false); }}>
                  <Users size={16} /> {t('joint')}
                </button>
              </div>
            )}
          </div>
        </div>
      </div>

      <label className="field">
        <span className="field-label"><Banknote size={16} /> {t('taxableIncome')}</span>
        <input
          type="number"
          min={0}
          step={100}
          value={income}
          onChange={(event) => onIncome(Math.max(0, Math.floor(Number(event.target.value) || 0)))}
        />
        <input
          type="range"
          min={0}
          max={500000}
          step={100}
          value={Math.min(income, 500000)}
          onChange={(event) => onIncome(Number(event.target.value))}
          aria-label="Taxable income slider"
        />
        <small>{grouped(income)} € taxable income, rounded down to whole euros for the tariff.</small>
      </label>
    </section>
  );
}
