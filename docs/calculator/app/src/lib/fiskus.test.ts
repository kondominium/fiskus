// Checks the calculator's core answers against the values the Rust oracle gives for the same inputs.
import { readFileSync } from 'node:fs';
import { describe, expect, it, beforeAll } from 'vitest';
import { Fiskus } from './fiskus';
import { euros, grouped, percent } from './format';

let fiskus: Fiskus;

beforeAll(async () => {
  const bytes = readFileSync(new URL('../../public/fiskus.wasm', import.meta.url));
  fiskus = await Fiskus.load(bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength));
});

describe('fiskus core', () => {
  it('lists every year with its basic allowance', () => {
    const meta = fiskus.meta();
    expect(meta.years).toHaveLength(21);
    expect(meta.years.at(-1)).toEqual({ year: 2026, basic_allowance_eur: 12348 });
  });

  it('computes the single filer tax at 30 000 € for VZ 2026 with the zone and rates', () => {
    const result = fiskus.explain(2026, 'single', 30000);
    expect(result.income_tax_eur).toBe(4217);
    expect(result.zone?.number).toBe(3);
    expect(result.marginal_rate).toBeCloseTo(0.2819, 4);
    expect(result.reference).toContain('bgbl-2024-I-449');
  });

  it('doubles the splitting half for a joint filer at 100 000 €', () => {
    expect(fiskus.explain(2026, 'joint', 100000).income_tax_eur).toBe(21096);
  });

  it('returns a curve with the zero zone at the start', () => {
    const curve = fiskus.curve(2026, 'single', 40000, 10000);
    expect(curve.points[0]).toEqual([0, 0]);
    expect(curve.points.at(-1)).toEqual([40000, 7209]);
  });

  it('compares every year for one income', () => {
    const compare = fiskus.compare('single', 30000);
    expect(compare.rows).toHaveLength(21);
    expect(compare.rows.find((row) => row.year === 2009)?.income_tax_eur).toBe(5698);
  });

  it('reports an error for a year that is not listed', () => {
    expect(() => fiskus.explain(2003, 'single', 1)).toThrow(/no tax year 2003/);
  });
});

describe('formats', () => {
  it('groups thousands with spaces and puts the euro sign after a space', () => {
    expect(euros(12348)).toBe('12 348 €');
    expect(grouped(-1234567)).toBe('−1 234 567');
    expect(percent(0.2819)).toBe('28.2 %');
  });
});
