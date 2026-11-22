import type { Explanation, ZoneTable } from '../lib/fiskus';
import { coefficient, euros, grouped } from '../lib/format';
import { Table2 } from 'lucide-react';
import { useI18n } from '../lib/i18n';
import { Formula } from './Formula';

// Every tariff zone of the year, with its formula as in § 32a Abs. 1 EStG. The active zone is marked.
export function Brackets({ table, active }: { table: ZoneTable; active: Explanation }) {
  const { t } = useI18n();
  return (
    <section className="card brackets" aria-label="Tax brackets">
      <h2><Table2 size={20} /> {t('allBrackets', { year: table.year })}</h2>
      <table>
        <thead>
          <tr>
            <th scope="col">Zone</th>
            <th scope="col">From taxable income</th>
            <th scope="col">Formula (x = whole-euro basis)</th>
          </tr>
        </thead>
        <tbody>
          <tr className={active.zone === null ? 'active' : ''}>
            <td>1</td>
            <td>{euros(0)}</td>
            <td>
              0 € up to {euros(table.basic_allowance_eur)} (Grundfreibetrag)
            </td>
          </tr>
          {table.zones.map((zone) => {
            const isQuadratic = zone.quadratic !== 0;
            const formula = isQuadratic
              ? `${coefficient(zone.quadratic)} · t² + ${coefficient(zone.linear)} · t${zone.constant ? ` + ${coefficient(zone.constant)}` : ''}, t = (x − ${grouped(zone.base_eur)}) / ${grouped(zone.step_eur)}`
              : `${coefficient(zone.linear)} · x ${zone.constant < 0 ? '−' : '+'} ${coefficient(Math.abs(zone.constant))}`;
            return (
              <tr key={zone.number} className={active.zone?.number === zone.number ? 'active' : ''}>
                <td>{zone.number}</td>
                <td>{euros(zone.from_eur)}</td>
                <td className="math">{formula}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
      <p className="muted">
        Tax is rounded down to whole euros at the end of each zone. Joint filers: the tariff applies to half of the joint
        income, and the result is doubled (§ 32a Abs. 5 EStG).
      </p>
      <Formula result={active} />
    </section>
  );
}
