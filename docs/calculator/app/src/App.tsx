import { useEffect, useMemo, useState } from 'react';
import type { Filing, Fiskus } from './lib/fiskus';
import { BarChart3, BookOpen, Calculator, HelpCircle, Receipt, Table2, TrendingUp } from 'lucide-react';
import { Brackets } from './components/Brackets';
import { useI18n } from './lib/i18n';
import { Chart } from './components/Chart';
import { FAQ } from './components/FAQ';
import { Comparison } from './components/Comparison';
import { Controls } from './components/Controls';
import { Formula } from './components/Formula';
import { Header } from './components/Header';
import { Provenance } from './components/Provenance';
import { Result } from './components/Result';

const CHART_MAX = 500000;
const CHART_STEP = 1000;

export function App({ fiskus }: { fiskus: Fiskus }) {
  const { t } = useI18n();
  const [activeSection, setActiveSection] = useState('result');
  const hideLogo = useMemo(
    () => new URLSearchParams(window.location.search).get('hideLogo') === 'true',
    [],
  );
  const initialParams = useMemo(() => new URLSearchParams(window.location.search), []);
  const meta = useMemo(() => fiskus.meta(), [fiskus]);
  const [year, setYear] = useState(Number(initialParams.get('year')) || 2026);
  const [filing, setFiling] = useState<Filing>((initialParams.get('filing') as Filing) || 'single');
  const [income, setIncome] = useState(Number(initialParams.get('income')) || 30000);

  useEffect(() => {
    const timer = window.setTimeout(() => {
      const url = new URL(window.location.href);
      url.searchParams.set('year', String(year));
      url.searchParams.set('filing', filing);
      url.searchParams.set('income', String(income));
      window.history.replaceState({}, '', url);
    }, 300);
    return () => window.clearTimeout(timer);
  }, [year, filing, income]);

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) setActiveSection(entry.target.id);
        });
      },
      { root: document.querySelector('.content'), threshold: 0.5 },
    );
    document.querySelectorAll('.content section[id]').forEach((el) => observer.observe(el));
    return () => observer.disconnect();
  }, []);

  const result = useMemo(() => fiskus.explain(year, filing, income), [fiskus, year, filing, income]);
  const table = useMemo(() => fiskus.zones(year), [fiskus, year]);
  const curve = useMemo(() => fiskus.curve(year, filing, CHART_MAX, CHART_STEP), [fiskus, year, filing]);
  const rates = useMemo(() => fiskus.rates(year, filing, CHART_MAX, CHART_STEP), [fiskus, year, filing]);
  const compare = useMemo(() => fiskus.compare(filing, income), [fiskus, filing, income]);
  const alternateTax = useMemo(() => {
    const other = filing === 'single' ? 'joint' : 'single';
    return fiskus.explain(year, other, income).income_tax_eur;
  }, [fiskus, filing, year, income]);

  return (
    <div className="page">
      <Header hideLogo={hideLogo} />
      <main className="layout">
        <aside className="sidebar">
          <div className="sidebar-sticky">
            <Controls
              years={meta.years}
              year={year}
              onYear={setYear}
              filing={filing}
              onFiling={setFiling}
              income={income}
              onIncome={setIncome}
            />
            <nav className="section-nav" aria-label="Sections">
              <a href="#result" className={activeSection === 'result' ? 'active' : ''}><Receipt size={16} /> {t('incomeTax', { year })}</a>
              <a href="#chart" className={activeSection === 'chart' ? 'active' : ''}><TrendingUp size={16} /> {t('tariffCurve')}</a>
              <a href="#formula" className={activeSection === 'formula' ? 'active' : ''}><Calculator size={16} /> {t('howTaxWorked')}</a>
              <a href="#brackets" className={activeSection === 'brackets' ? 'active' : ''}><Table2 size={16} /> {t('allBrackets', { year })}</a>
              <a href="#comparison" className={activeSection === 'comparison' ? 'active' : ''}><BarChart3 size={16} /> {t('sameIncome')}</a>
              <a href="#sources" className={activeSection === 'sources' ? 'active' : ''}><BookOpen size={16} /> {t('sources')}</a>
              <a href="#faq" className={activeSection === 'faq' ? 'active' : ''}><HelpCircle size={16} /> {t('faq')}</a>
            </nav>
          </div>
        </aside>
        <div className="content">
          <section id="result"><Result result={result} /></section>
          <section id="chart"><Chart curve={curve} rates={rates} income={result.taxable_income_eur} incomeTax={result.income_tax_eur} averageRate={result.average_rate} marginalRate={result.marginal_rate} /></section>
          <section id="brackets"><Brackets table={table} active={result} /></section>
          <section id="comparison"><Comparison compare={compare} year={year} /></section>
          <section id="sources"><Provenance result={result} /></section>
          <section id="faq">
            <FAQ
              result={result}
              jointTax={filing === 'single' ? alternateTax : undefined}
              singleTax={filing === 'joint' ? alternateTax : undefined}
            />
          </section>
        </div>
      </main>
      <footer className="footer">
        <p>
          {t('footer')}{' '}
          <a href="https://github.com/kondominium/fiskus" target="_blank" rel="noreferrer">
            github.com/kondominium/fiskus
          </a>.
        </p>
      </footer>
    </div>
  );
}
