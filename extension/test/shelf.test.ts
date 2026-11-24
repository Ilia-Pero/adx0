import assert from 'node:assert/strict';
import { test } from 'node:test';
import type { List } from '../src/lists.ts';
import type { Store, Taken, Web } from '../src/shelf.ts';
import { Shelf } from '../src/shelf.ts';

const LIST: List = {
  title: 'Sample',
  url: 'https://example.com/list.txt',
  format: 'hosts',
  cosmetic: false,
  languages: [],
};

/** The text of a list with the given number of addresses. */
function text(size: number): string {
  return Array.from({ length: size }, (_, index) => `0.0.0.0 ad${index}.com`).join('\n');
}

/** The internet that answers the given texts one after another. An empty text is a failure. */
class Pages implements Web {
  private readonly left: string[];

  constructor(...texts: ReadonlyArray<string>) {
    this.left = [...texts];
  }

  text(): Promise<string> {
    const next = this.left.shift() ?? '';
    return next === '' ? Promise.reject(new Error('404')) : Promise.resolve(next);
  }
}

/** A store that lives as long as the test. */
class Memory implements Store {
  private readonly values: Record<string, unknown> = {};

  get(): Promise<Record<string, unknown>> {
    return Promise.resolve(this.values);
  }

  set(values: Readonly<Record<string, unknown>>): Promise<void> {
    Object.assign(this.values, values);
    return Promise.resolve();
  }

  remove(): Promise<void> {
    return Promise.resolve();
  }
}

/** What the shelf gives the last time the list is taken from it. */
function last(...texts: ReadonlyArray<string>): Promise<Taken> {
  const shelf = new Shelf(new Pages(...texts), new Memory(), {
    silent: (list, reason) => `${list.title} is silent: ${reason}`,
    short: (list, rules) => `${list.title} is short: ${rules}`,
  });
  return texts.slice(1).reduce((earlier) => earlier.then(() => shelf.taken(LIST)), shelf.taken(LIST));
}

test('a downloaded list is taken as it is', async () => {
  assert.equal((await last(text(100))).filters.everywhere.size, 100);
});

test('a downloaded list gives no reason to complain', async () => {
  assert.deepEqual((await last(text(100))).failures, []);
});

test('the earlier list works while its site is down', async () => {
  assert.equal((await last(text(100), '')).filters.everywhere.size, 100);
});

test('a site that is down is mentioned', async () => {
  assert.deepEqual((await last('')).failures, ['Sample is silent: Error: 404']);
});

test('the earlier list works while its site sends garbage', async () => {
  assert.equal((await last(text(100), '<html>')).filters.everywhere.size, 100);
});

test('a list that is too short is mentioned', async () => {
  assert.deepEqual((await last(text(99))).failures, ['Sample is short: 99']);
});

test('a list that was never downloaded is empty', async () => {
  assert.equal((await last('')).filters.everywhere.size, 0);
});
