import type { Curve, RatesCurve } from '../lib/fiskus';
import { euros, grouped } from '../lib/format';
import { TrendingUp } from 'lucide-react';
import { useI18n } from '../lib/i18n';

const WIDTH = 640;
const HEIGHT = 360;
const MARGIN = { top: 24, right: 72, bottom: 48, left: 72 };

interface Props {
  curve: Curve;
  rates: RatesCurve;
  income: number;
  incomeTax: number;
  averageRate: number;
  marginalRate: number;
}

// Income tax (€, left axis) and average/marginal rates (%, right axis) against taxable income.
export function Chart({ curve, rates, income, incomeTax, averageRate, marginalRate }: Props) {
  const { t } = useI18n();
  const taxPoints = curve.points;
  const ratePoints = rates.points;
  const maxX = taxPoints.at(-1)?.[0] ?? 1;
  const maxTax = Math.max(...taxPoints.map(([, tax]) => tax), 1) * 1.08;
  const maxRate = 0.5;
  const plotWidth = WIDTH - MARGIN.left - MARGIN.right;
  const plotHeight = HEIGHT - MARGIN.top - MARGIN.bottom;
  const x = (value: number) => MARGIN.left + (value / maxX) * plotWidth;
  const yTax = (value: number) => MARGIN.top + plotHeight - (value / maxTax) * plotHeight;
  const yRate = (value: number) => MARGIN.top + plotHeight - (value / maxRate) * plotHeight;
  const taxPath = taxPoints
    .map(([euro, tax], index) => `${index === 0 ? 'M' : 'L'}${x(euro).toFixed(1)},${yTax(tax).toFixed(1)}`)
    .join(' ');
  const averagePath = ratePoints
    .map(([euro, average], index) => `${index === 0 ? 'M' : 'L'}${x(euro).toFixed(1)},${yRate(average).toFixed(1)}`)
    .join(' ');
  const marginalPath = ratePoints
    .map(([euro, , marginal], index) => `${index === 0 ? 'M' : 'L'}${x(euro).toFixed(1)},${yRate(marginal).toFixed(1)}`)
    .join(' ');

  const markerX = x(Math.min(income, maxX));
  const markerTaxY = yTax(incomeTax);
  const markerAvgY = yRate(averageRate);
  const markerMargY = yRate(marginalRate);

  return (
    <section className="card chart" aria-label="Tariff chart">
      <h2 className="chart-title"><TrendingUp size={18} /> {t('tariffCurve')}</h2>
      <svg viewBox={`0 0 ${WIDTH} ${HEIGHT}`} role="img" aria-labelledby="chart-title">
        <title id="chart-title">Income tax and rates against taxable income, VZ {curve.year}</title>
        <line x1={MARGIN.left} y1={MARGIN.top + plotHeight} x2={WIDTH - MARGIN.right} y2={MARGIN.top + plotHeight} className="axis" />
        <line x1={MARGIN.left} y1={MARGIN.top} x2={MARGIN.left} y2={MARGIN.top + plotHeight} className="axis" />
        <line x1={WIDTH - MARGIN.right} y1={MARGIN.top} x2={WIDTH - MARGIN.right} y2={MARGIN.top + plotHeight} className="axis" />
        {[0, 0.25, 0.5, 0.75, 1].map((fraction) => (
          <text key={`x${fraction}`} x={x(maxX * fraction)} y={HEIGHT - 20} textAnchor="middle" className="tick">
            {grouped(Math.round(maxX * fraction))}
          </text>
        ))}
        {[0, 0.5, 1].map((fraction) => (
          <text key={`yt${fraction}`} x={MARGIN.left - 8} y={yTax((maxTax / 1.08) * fraction) + 4} textAnchor="end" className="tick">
            {grouped(Math.round((maxTax / 1.08) * fraction))}
          </text>
        ))}
        {[0, 0.25, 0.5].map((fraction) => (
          <text key={`yr${fraction}`} x={WIDTH - MARGIN.right + 8} y={yRate(maxRate * fraction) + 4} textAnchor="start" className="tick">
            {Math.round(maxRate * fraction * 100)} %
          </text>
        ))}
        <path d={taxPath} className="curve">
          <title>Income tax curve against taxable income (VZ {curve.year})</title>
        </path>
        <path d={averagePath} style={{ stroke: 'var(--chart-ink, #0f3d2e)', strokeWidth: 1.5, fill: 'none', strokeDasharray: '6 4' }}>
          <title>Average rate curve (VZ {curve.year})</title>
        </path>
        <path d={marginalPath} style={{ stroke: '#e0391f', strokeWidth: 1.5, fill: 'none' }}>
          <title>Marginal rate curve (VZ {curve.year})</title>
        </path>
        <line x1={markerX} y1={markerTaxY} x2={markerX} y2={MARGIN.top + plotHeight} className="guide" />
        <circle cx={markerX} cy={markerTaxY} r={5} className="marker">
          <title>
            {euros(income)} → {euros(incomeTax)}; average {(averageRate * 100).toFixed(1)} %, marginal {(marginalRate * 100).toFixed(1)} %
          </title>
        </circle>
        <circle cx={markerX} cy={markerAvgY} r={4} style={{ fill: 'var(--chart-ink, #0f3d2e)' }}>
          <title>Average rate {(averageRate * 100).toFixed(2)} %</title>
        </circle>
        <circle cx={markerX} cy={markerMargY} r={4} style={{ fill: '#e0391f' }}>
          <title>Marginal rate {(marginalRate * 100).toFixed(2)} %</title>
        </circle>
        <text x={markerX + 10} y={markerTaxY - 10} className="marker-label">
          <tspan x={markerX + 10} dy="0">{euros(income)} → {euros(incomeTax)}</tspan>
          <tspan x={markerX + 10} dy="1.2em">
            avg {(averageRate * 100).toFixed(1)} %, marg {(marginalRate * 100).toFixed(1)} %
          </tspan>
        </text>
      </svg>
      <p className="muted">
        Left axis: income tax (€). Right axis: average and marginal rate (%). Taxable income (€) on the horizontal axis.
      </p>
    </section>
  );
}
