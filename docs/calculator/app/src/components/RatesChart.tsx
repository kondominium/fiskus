import type { RatesCurve } from '../lib/fiskus';
import { grouped } from '../lib/format';

const WIDTH = 640;
const HEIGHT = 360;
const MARGIN = { top: 24, right: 24, bottom: 48, left: 72 };

interface Props {
  curve: RatesCurve;
}

// Average and marginal tax rates against taxable income for the selected year and filing.
export function RatesChart({ curve }: Props) {
  const points = curve.points;
  const maxX = points.at(-1)?.[0] ?? 1;
  const maxY = 0.5;
  const plotWidth = WIDTH - MARGIN.left - MARGIN.right;
  const plotHeight = HEIGHT - MARGIN.top - MARGIN.bottom;
  const x = (value: number) => MARGIN.left + (value / maxX) * plotWidth;
  const y = (value: number) => MARGIN.top + plotHeight - (value / maxY) * plotHeight;
  const averagePath = points
    .map(([euro, average], index) => `${index === 0 ? 'M' : 'L'}${x(euro).toFixed(1)},${y(average).toFixed(1)}`)
    .join(' ');
  const marginalPath = points
    .map(([euro, , marginal], index) => `${index === 0 ? 'M' : 'L'}${x(euro).toFixed(1)},${y(marginal).toFixed(1)}`)
    .join(' ');

  return (
    <section className="card chart" aria-label="Tax rates chart">
      <h2>Tax rates</h2>
      <svg viewBox={`0 0 ${WIDTH} ${HEIGHT}`} role="img" aria-labelledby="rates-title">
        <title id="rates-title">Average and marginal tax rates against taxable income, VZ {curve.year}</title>
        <line x1={MARGIN.left} y1={MARGIN.top + plotHeight} x2={WIDTH - MARGIN.right} y2={MARGIN.top + plotHeight} className="axis" />
        <line x1={MARGIN.left} y1={MARGIN.top} x2={MARGIN.left} y2={MARGIN.top + plotHeight} className="axis" />
        {[0, 0.25, 0.5, 0.75, 1].map((fraction) => (
          <text key={`x${fraction}`} x={x(maxX * fraction)} y={HEIGHT - 20} textAnchor="middle" className="tick">
            {grouped(Math.round(maxX * fraction))}
          </text>
        ))}
        {[0, 0.25, 0.5].map((fraction) => (
          <text key={`y${fraction}`} x={MARGIN.left - 8} y={y(maxY * fraction) + 4} textAnchor="end" className="tick">
            {Math.round(maxY * fraction * 100)} %
          </text>
        ))}
        <path d={averagePath} className="curve" />
        <path d={marginalPath} className="guide" style={{ stroke: '#e0391f', strokeWidth: 2, fill: 'none' }} />
      </svg>
      <p className="muted">
        Average rate (tariff curve colour) and marginal rate (signal colour) as a share of taxable income.
      </p>
    </section>
  );
}
