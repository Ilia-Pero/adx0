/** The site a page belongs to. */
export class Site {
  private readonly host: string;

  constructor(host: string) {
    this.host = host;
  }

  /** The name of the site and of all sites it is a part of: a.b.com, b.com. */
  names(): ReadonlyArray<string> {
    const parts = this.host.toLowerCase().replace(/^www\./, '').split('.');
    return parts
      .map((_, index) => parts.slice(index).join('.'))
      .filter((name) => name.includes('.'));
  }

  /** True if the site, or one it is a part of, is in the list. */
  within(list: ReadonlyArray<string>): boolean {
    return this.names().some((name) => list.includes(name));
  }
}

/**
 * CSS that hides ads. Every selector has a rule of its own: if the browser does not understand
 * one of them, the others go on working.
 */
export function sheet(
  selectors: ReadonlyArray<string>,
  spared: ReadonlyArray<string>,
): string {
  const skip = new Set(spared);
  return Array.from(new Set(selectors))
    .filter((selector) => !skip.has(selector))
    .map((selector) => `${selector}{display:none!important}`)
    .join('\n');
}
