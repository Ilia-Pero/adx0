/**
 * Chromium does not know Belarusian and Kazakh. They write numbers and days with digits as Russian
 * does.
 */
const SPARE = 'ru';

/** How a language writes numbers and days. */
export class Regional {
  private readonly code: string;

  constructor(code: string) {
    this.code = code;
  }

  number(value: number): string {
    return value.toLocaleString(this.known() ? this.code : SPARE);
  }

  /** The day of the moment, which is in milliseconds since 1970. */
  day(moment: number): string {
    return this.known()
      ? new Date(moment).toLocaleDateString(this.code, {
          day: 'numeric',
          month: 'long',
          year: 'numeric',
        })
      : new Date(moment).toLocaleDateString(SPARE, {
          day: '2-digit',
          month: '2-digit',
          year: 'numeric',
        });
  }

  /** Whether the browser can write in the language. Chromium calls months in Kazakh M01 and so on. */
  private known(): boolean {
    return (
      Intl.DateTimeFormat.supportedLocalesOf(this.code).length > 0 &&
      !/^M\d/.test(new Date(0).toLocaleDateString(this.code, { month: 'long' }))
    );
  }
}
