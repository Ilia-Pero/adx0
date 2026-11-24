import type { Filters } from './filters.ts';
import { FilterText, count } from './filters.ts';
import type { List } from './lists.ts';

/** The internet lists are downloaded from. */
export interface Web {
  text(url: string): Promise<string>;
}

/** Where things are kept while the browser is closed. */
export interface Store {
  get(keys: ReadonlyArray<string>): Promise<Record<string, unknown>>;
  set(values: Readonly<Record<string, unknown>>): Promise<void>;
  remove(keys: ReadonlyArray<string>): Promise<void>;
}

/** What the shelf says when a list cannot be updated. */
export interface Complaints {
  silent(list: List, reason: string): string;
  short(list: List, rules: number): string;
}

/** Filters of a list and the reasons why it was not updated, if it was not. */
export interface Taken {
  readonly filters: Filters;
  readonly failures: ReadonlyArray<string>;
}

/** With fewer rules than this, something else came instead of a list, such as an error page. */
const FEWEST = 100;

/** Under which key the text of the list is kept. */
export function place(list: List): string {
  return `list:${list.title}`;
}

/**
 * A shelf with downloaded lists. The last good text of each list stays on it, so blocking does
 * not depend on whether its site is up.
 */
export class Shelf {
  private readonly web: Web;
  private readonly store: Store;
  private readonly complaints: Complaints;

  constructor(web: Web, store: Store, complaints: Complaints) {
    this.web = web;
    this.store = store;
    this.complaints = complaints;
  }

  /** The list: just downloaded, or the last good one. */
  async taken(list: List): Promise<Taken> {
    try {
      const text = await this.web.text(list.url);
      const filters = read(text, list);
      const rules = count(filters);
      if (rules < FEWEST) {
        return this.kept(list, this.complaints.short(list, rules));
      }
      await this.store.set({ [place(list)]: text });
      return { filters, failures: [] };
    } catch (error: unknown) {
      return this.kept(list, this.complaints.silent(list, String(error)));
    }
  }

  private async kept(list: List, failure: string): Promise<Taken> {
    const text = (await this.store.get([place(list)]))[place(list)];
    return { filters: read(typeof text === 'string' ? text : '', list), failures: [failure] };
  }
}

function read(text: string, list: List): Filters {
  return new FilterText(text, list.format, list.cosmetic).filters();
}
