// Number formats of the language rules: a space between thousands, a point for decimals, and the
// euro sign after a space (12 348 €).

export function grouped(value: number, decimals = 0): string {
  const negative = value < 0;
  const fixed = Math.abs(value).toFixed(decimals);
  const [whole, fraction] = fixed.split('.');
  const groupedWhole = whole.replace(/\B(?=(\d{3})+(?!\d))/g, ' ');
  const body = fraction ? `${groupedWhole}.${fraction}` : groupedWhole;
  return negative ? `−${body}` : body;
}

export function euros(value: number, decimals = 0): string {
  return `${grouped(value, decimals)} €`;
}

export function percent(fraction: number, decimals = 1): string {
  return `${(fraction * 100).toFixed(decimals)} %`;
}

export function coefficient(value: number): string {
  return grouped(value, Number.isInteger(value) ? 0 : 2);
}
