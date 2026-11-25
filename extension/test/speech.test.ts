import assert from 'node:assert/strict';
import { readdirSync } from 'node:fs';
import { test } from 'node:test';
import type { Store } from '../src/shelf.ts';
import type { Speech } from '../src/speech.ts';
import { Chosen, TONGUES, choice, choose, spoken } from '../src/speech.ts';

const browsers: Speech = { text: () => 'the language of the browser' };

/** A store that keeps everything in memory. */
function memory(kept: Readonly<Record<string, unknown>>): Store {
  let values: Record<string, unknown> = { ...kept };
  return {
    get: () => Promise.resolve(values),
    set: (fresh) => {
      values = { ...values, ...fresh };
      return Promise.resolve();
    },
    remove: () => Promise.resolve(),
  };
}

/** What the popup says when the store holds the choice. */
async function said(kept: Readonly<Record<string, unknown>>): Promise<string> {
  const speech = await spoken(memory(kept), browsers, (path) =>
    Promise.resolve({ blocked: { message: `the text from ${path}` } }),
  );
  return speech.text('blocked', []);
}

test('every language has a folder with messages, and every folder is in the list', () => {
  assert.deepEqual(
    TONGUES.map((tongue) => tongue.code),
    readdirSync(new URL('../web/_locales/', import.meta.url)).toSorted(),
  );
});

test('without a choice, the language of the browser is spoken', async () => {
  assert.equal(await said({}), 'the language of the browser');
});

test('the chosen language is spoken', async () => {
  assert.equal(await said({ language: 'el' }), 'the text from _locales/el/messages.json');
});

test('an unknown choice leaves the language to the browser', async () => {
  assert.equal(await said({ language: '../secret' }), 'the language of the browser');
});

test('the choice is remembered', async () => {
  const store = memory({});
  await choose(store, 'ja');
  assert.equal(await choice(store), 'ja');
});

test('holes in a message are filled', () => {
  assert.equal(
    new Chosen({ lists: { message: '$1 of $2' } }).text('lists', ['three', 'five']),
    'three of five',
  );
});

test('a key without a message is shown as it is', () => {
  assert.equal(new Chosen({}).text('lists', []), 'lists');
});
