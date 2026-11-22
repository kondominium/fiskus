// Typed client for the fiskus WebAssembly bridge (docs/calculator/wasm). The core computes every number.

export type Filing = 'single' | 'joint';

export interface YearInfo {
  year: number;
  basic_allowance_eur: number;
}

export interface Meta {
  years: YearInfo[];
  filings: Filing[];
}

export interface Zone {
  number: number;
  from_eur: number;
  base_eur: number;
  step_eur: number;
  quadratic: number;
  linear: number;
  constant: number;
}

export interface Explanation {
  year: number;
  filing: Filing;
  taxable_income_eur: number;
  basis_eur: number;
  basic_allowance_eur: number;
  zone: Zone | null;
  unrounded_basis_tax: number;
  basis_tax_eur: number;
  income_tax_eur: number;
  average_rate: number;
  marginal_rate: number;
  citation: string;
  source: string;
  reference: string;
}

export interface ZoneTable {
  year: number;
  basic_allowance_eur: number;
  zones: Zone[];
  citation: string;
  source: string;
  reference: string;
}

export interface Curve {
  year: number;
  points: [number, number][];
}

export interface RatesCurve {
  year: number;
  points: [number, number, number][];
}

export interface CompareRow {
  year: number;
  basic_allowance_eur: number;
  income_tax_eur: number; average_rate: number;
}

export interface Compare {
  filing: Filing;
  euros: number;
  rows: CompareRow[];
}

type Request = Record<string, unknown>;

interface Exports {
  memory: WebAssembly.Memory;
  fiskus_alloc: (length: number) => number;
  fiskus_free: (pointer: number, length: number) => void;
  fiskus_call: (pointer: number, length: number) => bigint;
}

export class Fiskus {
  private readonly exports: Exports;

  private constructor(exports: Exports) {
    this.exports = exports;
  }

  static async load(source: ArrayBuffer | Response): Promise<Fiskus> {
    const bytes = source instanceof Response ? await source.arrayBuffer() : source;
    const { instance } = await WebAssembly.instantiate(bytes, {});
    return new Fiskus(instance.exports as unknown as Exports);
  }

  call<T>(request: Request): T {
    const { memory, fiskus_alloc, fiskus_free, fiskus_call } = this.exports;
    const input = new TextEncoder().encode(JSON.stringify(request));
    const pointer = fiskus_alloc(input.length);
    new Uint8Array(memory.buffer, pointer, input.length).set(input);
    const packed = fiskus_call(pointer, input.length);
    fiskus_free(pointer, input.length);

    const outputPointer = Number(packed >> 32n);
    const outputLength = Number(packed & 0xffffffffn);
    const text = new TextDecoder().decode(
      new Uint8Array(memory.buffer, outputPointer, outputLength).slice(),
    );
    fiskus_free(outputPointer, outputLength);

    const parsed = JSON.parse(text) as T & { error?: string };
    if (parsed && typeof parsed === 'object' && 'error' in parsed && parsed.error) {
      throw new Error(parsed.error);
    }
    return parsed;
  }

  meta(): Meta {
    return this.call<Meta>({ op: 'meta' });
  }

  explain(year: number, filing: Filing, euros: number): Explanation {
    return this.call<Explanation>({ op: 'explain', year, filing, euros });
  }

  zones(year: number): ZoneTable {
    return this.call<ZoneTable>({ op: 'zones', year, filing: 'single', euros: 0 });
  }

  curve(year: number, filing: Filing, max: number, step: number): Curve {
    return this.call<Curve>({ op: 'curve', year, filing, max, step });
  }
  rates(year: number, filing: Filing, max: number, step: number): RatesCurve {
    return this.call<RatesCurve>({ op: 'rates', year, filing, max, step });
  }

  compare(filing: Filing, euros: number): Compare {
    return this.call<Compare>({ op: 'compare', filing, euros });
  }
}
