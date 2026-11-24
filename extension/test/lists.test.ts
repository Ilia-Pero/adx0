import assert from 'node:assert/strict';
import { test } from 'node:test';
import { suitable } from '../src/lists.ts';

function titles(tags: ReadonlyArray<string>): ReadonlyArray<string> {
  return suitable(tags).map((list) => list.title);
}

test('lists for everyone are for those who speak any language', () => {
  assert.deepEqual(titles(['sr']), ['Peter Lowe', 'AdAway', 'EasyList']);
});

test('a regional list is for those who read its language', () => {
  assert.equal(titles(['ru-RU', 'en']).includes('RU AdList'), true);
});

test('a regional list is not for others', () => {
  assert.equal(titles(['en-US']).includes('RU AdList'), false);
});

test('every language brings its list', () => {
  assert.deepEqual(titles(['de', 'fr-CH']).slice(3), ['EasyList Germany', 'Liste FR']);
});

test('every list is downloaded through an encrypted connection', () => {
  assert.equal(
    suitable(['de', 'el', 'es', 'fr', 'he', 'it', 'ja', 'ko', 'ru', 'zh']).every((list) =>
      list.url.startsWith('https://'),
    ),
    true,
  );
});
