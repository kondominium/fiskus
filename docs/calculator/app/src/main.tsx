import { StrictMode, useEffect, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from './App';
import { Fiskus } from './lib/fiskus';
import { LanguageProvider } from './lib/i18n';
import './styles.css';
import 'flag-icons/css/flag-icons.min.css';

function Root() {
  const [state, setState] = useState<{ fiskus?: Fiskus; error?: string }>({});

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const response = await fetch(`${import.meta.env.BASE_URL}fiskus.wasm`);
        const fiskus = await Fiskus.load(response);
        if (!cancelled) setState({ fiskus });
      } catch (error) {
        if (!cancelled) setState({ error: String(error) });
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  if (state.error) return <p role="alert" className="loading">Could not load the calculator: {state.error}</p>;
  if (!state.fiskus) return <p className="loading">Loading the calculator…</p>;
  return <LanguageProvider><App fiskus={state.fiskus} /></LanguageProvider>;
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <Root />
  </StrictMode>,
);
