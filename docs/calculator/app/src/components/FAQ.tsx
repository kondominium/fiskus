import type { Explanation } from '../lib/fiskus';
import { euros, percent } from '../lib/format';
import { HelpCircle } from 'lucide-react';
import { useI18n } from '../lib/i18n';

interface Props {
  result: Explanation;
  jointTax?: number;
  singleTax?: number;
}

export function FAQ({ result, jointTax, singleTax }: Props) {
  const { t } = useI18n();
  const inZeroZone = result.zone === null;
  const items = [
    {
      q: t('faqQ1', { income: euros(result.taxable_income_eur), year: result.year }),
      a: t('faqA1', { year: result.year, filing: result.filing, income: euros(result.taxable_income_eur), tax: euros(result.income_tax_eur), avg: percent(result.average_rate), marginal: inZeroZone ? '0 %' : percent(result.marginal_rate) }),
    },
    {
      q: t('faqQ2'),
      a: t('faqA2', { avg: percent(result.average_rate), marginal: inZeroZone ? '0 %' : percent(result.marginal_rate) }),
    },
    {
      q: t('faqQ3'),
      a: inZeroZone
        ? t('faqA3Zero', { allowance: euros(result.basic_allowance_eur) })
        : t('faqA3Zone', { zone: result.zone?.number ?? 0, from: euros(result.zone!.from_eur), basis: euros(result.basis_eur) }),
    },
    {
      q: t('faqQ4'),
      a: t('faqA4', { year: result.year, allowance: euros(result.basic_allowance_eur) }),
    },
    jointTax !== undefined
      ? {
          q: t('faqQ5Joint'),
          a: t('faqA5Joint', { income: euros(result.taxable_income_eur), jointTax: euros(jointTax), tax: euros(result.income_tax_eur) }),
        }
      : singleTax !== undefined
        ? {
            q: t('faqQ5Single'),
            a: t('faqA5Single', { income: euros(result.taxable_income_eur), singleTax: euros(singleTax), tax: euros(result.income_tax_eur) }),
          }
        : {
            q: t('faqQ5How'),
            a: t('faqA5How', { basis: euros(result.basis_eur), allowance: euros(result.basic_allowance_eur), zone: result.zone?.number ?? 1, basisTax: euros(result.basis_tax_eur) }),
          },
  ];

  return (
    <section className="card faq" aria-label="Frequently asked questions" id="faq">
      <h2><HelpCircle size={20} /> {t('faq')}</h2>
      <div className="faq-list">
        {items.map((item, index) => (
          <details key={index} className="faq-item" open>
            <summary>{item.q}</summary>
            <p>{item.a}</p>
          </details>
        ))}
      </div>
    </section>
  );
}
