import type { Report, Row } from './backend.ts';
import { Regional } from './regional.ts';
import { filled } from './words.ts';

/** A day in the given language. */
function day(seconds: number, code: string): string {
  return new Regional(code).day(seconds * 1000);
}

/**
 * Sentences one after another. Some languages end a date with a dot of its own, and some
 * do not put spaces between sentences.
 */
function text(sentences: ReadonlyArray<string>): string {
  return sentences.join(' ').replace(/\.\./g, '.').replace(/。 /g, '。');
}

/** The name of the program as a formula: by what an ad is multiplied now. */
export function formula(report: Report): string {
  return report.count === 0 ? 'ad × 1 = ad' : 'ad × 0 = 0';
}

/** What is written under the title. */
export function summary(report: Report): string {
  const words = report.words;
  if (report.count === 0) {
    return words['note.off'];
  }
  const moments = report.rows.filter((row) => row.on && row.updated > 0).map((row) => row.updated);
  return text([
    filled(words['note.on'], 'count', new Regional(report.spoken).number(report.count)),
    ...moments
      .slice(0, 1)
      .map(() => filled(words['note.date'], 'date', day(Math.min(...moments), report.spoken))),
    words['note.restart'],
  ]);
}

/** When the list was downloaded. */
export function age(row: Row, report: Report): string {
  return row.updated === 0
    ? report.words['list.never']
    : filled(report.words['list.date'], 'date', day(row.updated, report.spoken));
}
