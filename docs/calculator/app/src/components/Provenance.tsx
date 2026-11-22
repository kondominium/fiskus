import type { Explanation } from '../lib/fiskus';
import { BookOpen } from 'lucide-react';
import { useI18n } from '../lib/i18n';

// Where the numbers come from: the statute citation and the archived source of the year's tariff.
export function Provenance({ result }: { result: Explanation }) {
  const { t } = useI18n();
  return (
    <section className="card provenance" aria-label="Sources">
      <h2><BookOpen size={20} /> {t('sources')}</h2>
      <p>
        <strong>{t('citation')}:</strong> {result.citation}
      </p>
      <p>
        <strong>{t('source')}:</strong>{' '}
        <a href={result.source} target="_blank" rel="noreferrer">
          {result.source}
        </a>
      </p>
      <p>
        <strong>{t('archivedCopy')}:</strong> <code>{result.reference}</code>
      </p>
    </section>
  );
}
