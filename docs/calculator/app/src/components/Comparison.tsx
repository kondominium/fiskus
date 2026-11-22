import { useState } from 'react';
import type { Compare } from '../lib/fiskus';
import { euros, percent } from '../lib/format';
import { BarChart3 } from 'lucide-react';
import { useI18n } from '../lib/i18n';

// The same income under every listed year, so changes between years can be checked.
export function Comparison({ compare, year }: { compare: Compare; year: number }) {
  const { t } = useI18n();
  const [hovered, setHovered] = useState<number | null>(null);
  const taxes = compare.rows.map((row) => row.income_tax_eur);
  const maxTax = Math.max(...taxes, 1);
  const width = 520;
  const height = 180;
  const margin = { top: 12, right: 40, bottom: 28, left: 12 };
  const plotWidth = width - margin.left - margin.right;
  const plotHeight = height - margin.top - margin.bottom;
  const barWidth = plotWidth / compare.rows.length;
  const x = (index: number) => margin.left + index * barWidth;
  const yTax = (tax: number) => margin.top + plotHeight - (tax / maxTax) * plotHeight;
  const yRate = (rate: number) => margin.top + plotHeight - (rate / 0.5) * plotHeight;
  const ratePath = compare.rows
    .map((row, index) => `${index === 0 ? 'M' : 'L'}${(x(index) + barWidth / 2).toFixed(1)},${yRate(row.average_rate).toFixed(1)}`)
    .join(' ');

  return (
    <section className="card comparison" aria-label="Comparison by year">
      <h2><BarChart3 size={20} /> {t('sameIncome')}</h2>
      <div className="comparison-chart-wrap">
        <svg viewBox={`0 0 ${width} ${height}`} className="comparison-chart" role="img" aria-label="Income tax by year">
        {compare.rows.map((row, index) => {
          const barHeight = (row.income_tax_eur / maxTax) * plotHeight;
          const bx = x(index) + barWidth * 0.15;
          const bw = barWidth * 0.7;
          const by = margin.top + plotHeight - barHeight;
          return (
            <g key={row.year}>
              <rect
                x={bx}
                y={by}
                width={bw}
                height={barHeight}
                fill={row.year === year ? 'var(--accent)' : 'var(--border)'}
                rx={3}
                style={{ cursor: 'pointer' }}
                onMouseEnter={() => setHovered(index)}
                onMouseLeave={() => setHovered(null)}
              >
                <title>{`VZ ${row.year}: ${euros(row.income_tax_eur)} tax, ${percent(row.average_rate)} average, allowance ${euros(row.basic_allowance_eur)}`}</title>
              </rect>
            </g>
          );
        })}
        <path d={ratePath} fill="none" stroke="#e0391f" strokeWidth={1.5} />
        {compare.rows.map((row, index) => (
          <circle
            key={`r${row.year}`}
            cx={x(index) + barWidth / 2}
            cy={yRate(row.average_rate)}
            r={4}
            fill="#e0391f"
            onMouseEnter={() => setHovered(index)}
            onMouseLeave={() => setHovered(null)}
            style={{ cursor: 'pointer' }}
          />
        ))}
        {compare.rows.map((row, index) =>
          row.year === year ? (
            <text key={`label${row.year}`} x={x(index) + barWidth / 2} y={yTax(row.income_tax_eur) - 8} textAnchor="middle" fontSize={10} fill="var(--ink)" fontWeight="600">
              {euros(row.income_tax_eur)}
            </text>
          ) : null,
        )}
        {compare.rows.map((row, index) => (
          <text key={`t${row.year}`} x={x(index) + barWidth / 2} y={height - 8} textAnchor="middle" fontSize={9} fill="var(--muted)">
            {row.year}
          </text>
        ))}
      </svg>
      {(() => {
        const activeIndex = hovered !== null ? hovered : compare.rows.findIndex((r) => r.year === year);
        if (activeIndex < 0) return null;
        const row = compare.rows[activeIndex];
        const prev = activeIndex > 0 ? compare.rows[activeIndex - 1] : null;
        const delta = prev ? row.income_tax_eur - prev.income_tax_eur : null;
        const pct = prev && prev.income_tax_eur !== 0 ? (delta! / prev.income_tax_eur) * 100 : null;
        return (
          <div className="comparison-tooltip" style={{ left: `${((x(activeIndex) + barWidth / 2) / width) * 100}%`, transform: "translateX(-50%)", top: "0.5rem" }}>
            <strong>VZ {row.year}</strong>
            <div>Tax: {euros(row.income_tax_eur)}</div>
            <div>Average rate: {percent(row.average_rate)}</div>
            {prev && (
              <div>
                vs VZ {prev.year}: {delta! >= 0 ? '+' : ''}{euros(delta!)} {pct !== null ? `(${pct.toFixed(1)} %)` : ''}
              </div>
            )}
          </div>
        );
      })()}
      </div>
      <div className="scroll">
        <table>
          <thead>
            <tr>
              <th scope="col">VZ</th>
              <th scope="col">Basic allowance</th>
              <th scope="col">Income tax</th>
              <th scope="col">Average rate</th>
            </tr>
          </thead>
          <tbody>
            {[...compare.rows].reverse().map((row) => (
              <tr key={row.year} className={row.year === year ? 'active' : ''}>
                <td>{row.year}</td>
                <td>{euros(row.basic_allowance_eur)}</td>
                <td>{euros(row.income_tax_eur)}</td>
                <td>{percent(row.average_rate)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}