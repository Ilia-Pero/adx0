import type { Format } from './lists.ts';

/** What to block and hide according to one or several lists. */
export interface Filters {
  /** Sites from which nothing is loaded. */
  readonly everywhere: ReadonlySet<string>;
  /** Sites from which nothing is loaded onto pages of other sites. */
  readonly foreign: ReadonlySet<string>;
  /** Selectors of ads on all pages. */
  readonly generic: ReadonlySet<string>;
  /** Selectors of ads by sites. */
  readonly sites: ReadonlyMap<string, ReadonlySet<string>>;
  /** Selectors that must not be used on these sites. */
  readonly spared: ReadonlyMap<string, ReadonlySet<string>>;
}

const NOWHERE: ReadonlySet<string> = new Set(['0.0.0.0', '127.0.0.1', '::', '::1']);

const NAME = /^(?=.{1,253}$)(?:[a-z0-9_](?:[a-z0-9_-]{0,61}[a-z0-9_])?\.)+[a-z][a-z0-9-]{0,61}[a-z0-9]$/;

/** What plain CSS does not have: such rules work in large ad blockers only. */
const EXTENDED =
  /^\+js|^\^|:(?:-abp-|has-text|contains|xpath|matches-|upward|remove|style|if|if-not|min-text|nth-ancestor|watch-attr|others|properties)\b/;

/** Characters with which a list could end the selector and write its own CSS into the page. */
const FOREIGN = /[{}<\\]|\/\*|\*\/|@import|url\(/i;

function put(map: Map<string, Set<string>>, key: string, value: string): void {
  const known = map.get(key);
  if (known === undefined) {
    map.set(key, new Set([value]));
  } else {
    known.add(value);
  }
}

/** The text of a list. */
export class FilterText {
  private readonly text: string;
  private readonly format: Format;
  private readonly cosmetic: boolean;

  /** Cosmetic says whether to take the rules that hide elements on pages. */
  constructor(text: string, format: Format, cosmetic: boolean) {
    this.text = text;
    this.format = format;
    this.cosmetic = cosmetic;
  }

  filters(): Filters {
    const everywhere = new Set<string>();
    const foreign = new Set<string>();
    const generic = new Set<string>();
    const sites = new Map<string, Set<string>>();
    const spared = new Map<string, Set<string>>();
    for (const raw of this.text.split('\n')) {
      const line = raw.trim();
      if (this.format === 'hosts') {
        this.row(line).forEach((name) => everywhere.add(name));
        continue;
      }
      const hiding = /^([^#]*)#(@?)#(.+)$/.exec(line);
      if (hiding !== null) {
        if (this.cosmetic) {
          this.hide(hiding[1] ?? '', hiding[2] === '@', hiding[3] ?? '', generic, sites, spared);
        }
        continue;
      }
      const rule = /^\|\|([^^$/*]+)\^(\$third-party|\$3p)?$/.exec(line);
      if (rule !== null && NAME.test(rule[1] ?? '')) {
        (rule[2] === undefined ? everywhere : foreign).add(rule[1] ?? '');
      }
    }
    return { everywhere, foreign, generic, sites, spared };
  }

  /**
   * Names in a line of the hosts format. A comment starts with # after a space. In
   * site.com#?#.banner there is no comment: otherwise the site itself would be blocked.
   */
  private row(line: string): ReadonlyArray<string> {
    const all = line.toLowerCase().split(/\s+/).filter((word) => word !== '');
    const note = all.findIndex((word) => /^[#![@]/.test(word));
    const words = note < 0 ? all : all.slice(0, note);
    if (words.length === 1) {
      return words.filter((word) => NAME.test(word));
    }
    if (words.length > 1 && NOWHERE.has(words[0] ?? '')) {
      return words.slice(1).filter((word) => NAME.test(word));
    }
    return [];
  }

  private hide(
    scope: string,
    exception: boolean,
    selector: string,
    generic: Set<string>,
    sites: Map<string, Set<string>>,
    spared: Map<string, Set<string>>,
  ): void {
    if (EXTENDED.test(selector) || FOREIGN.test(selector)) {
      return;
    }
    const names = scope
      .toLowerCase()
      .split(',')
      .filter((name) => name !== '');
    const on = names.filter((name) => NAME.test(name));
    const off = names
      .filter((name) => name.startsWith('~'))
      .map((name) => name.slice(1))
      .filter((name) => NAME.test(name));
    if (exception) {
      on.forEach((name) => put(spared, name, selector));
      return;
    }
    on.forEach((name) => put(sites, name, selector));
    if (on.length === 0 && names.length === off.length) {
      generic.add(selector);
      off.forEach((name) => put(spared, name, selector));
    }
  }
}

/** How many rules there are in the filters. */
export function count(filters: Filters): number {
  return (
    filters.everywhere.size + filters.foreign.size + filters.generic.size + filters.sites.size
  );
}

function union<T>(sets: ReadonlyArray<ReadonlySet<T>>): ReadonlySet<T> {
  return new Set(sets.flatMap((set) => Array.from(set)));
}

function merged(
  maps: ReadonlyArray<ReadonlyMap<string, ReadonlySet<string>>>,
): ReadonlyMap<string, ReadonlySet<string>> {
  const out = new Map<string, Set<string>>();
  maps.forEach((map) =>
    map.forEach((values, key) => values.forEach((value) => put(out, key, value))),
  );
  return out;
}

/** Several lists together. */
export function joined(all: ReadonlyArray<Filters>): Filters {
  return {
    everywhere: union(all.map((one) => one.everywhere)),
    foreign: union(all.map((one) => one.foreign)),
    generic: union(all.map((one) => one.generic)),
    sites: merged(all.map((one) => one.sites)),
    spared: merged(all.map((one) => one.spared)),
  };
}
