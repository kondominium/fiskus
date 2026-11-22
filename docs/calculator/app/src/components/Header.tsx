import { useState } from 'react';
import { useI18n } from '../lib/i18n';

const languageNames: Record<string, { code: string; label: string }> = {
  en: { code: 'gb', label: 'English' },
  de: { code: 'de', label: 'Deutsch' },
  pt: { code: 'pt', label: 'Português' },
  'pt-BR': { code: 'br', label: 'Português (Brasil)' },
  it: { code: 'it', label: 'Italiano' },
};

export function Header({ hideLogo = false }: { hideLogo?: boolean }) {
  const { lang, setLang, t } = useI18n();
  const [open, setOpen] = useState(false);
  return (
    <header className="header">
      {!hideLogo && (
        <div className="brand">
          <picture>
            <source srcSet="logo/fiskus-dark-mode.svg" media="(prefers-color-scheme: dark)" />
            <img src="logo/fiskus-light-mode.svg" alt="fiskus" height={73} />
          </picture>
          <p className="tagline">{t('tagline')}</p>
        </div>
      )}
      <div className="lang-dropdown" aria-label="Language">
        <button type="button" className="lang-trigger" onClick={() => setOpen((v) => !v)}>
          <span className={`fi fi-${languageNames[lang].code}`} />
          <span>{languageNames[lang].label}</span>
        </button>
        {open && (
          <div className="lang-menu">
            {(['en', 'de', 'pt', 'pt-BR', 'it'] as const).map((code) => (
              <button
                key={code}
                type="button"
                className={code === lang ? 'active' : ''}
                onClick={() => {
                  setLang(code);
                  setOpen(false);
                }}
              >
                <span className={`fi fi-${languageNames[code].code}`} />
                <span>{languageNames[code].label}</span>
              </button>
            ))}
          </div>
        )}
      </div>
    </header>
  );
}
