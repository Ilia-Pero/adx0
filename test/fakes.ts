import type { Report, Row } from '../src/backend.ts';
import { code, spoken } from '../src/languages.ts';

/** A list that was downloaded at the given second. */
export function row(id: string, on: boolean, updated: number): Row {
  return { id, title: id, note: '', on, updated };
}

/** A report in the language with the given number of blocked addresses. */
export function report(tag: string, count: number, rows: ReadonlyArray<Row>): Report {
  return {
    count,
    rows,
    settings: { sources: [], blocked: ['b.com'], allowed: ['a.com'], language: '', fresh: true },
    spoken: code(tag),
    languages: [],
    words: spoken(tag),
    version: '0.0.0',
  };
}
