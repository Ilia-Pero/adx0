import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import { test } from 'node:test';

type Messages = Readonly<Record<string, { readonly message: string }>>;

const ROOT = new URL('../web/_locales/', import.meta.url);

function messages(name: string): Messages {
  return JSON.parse(readFileSync(new URL(`${name}/messages.json`, ROOT), 'utf8'));
}

/** Languages for which the statement about English messages is wrong. */
function failing(agrees: (own: Messages, english: Messages) => boolean): ReadonlyArray<string> {
  return readdirSync(ROOT).filter((name) => !agrees(messages(name), messages('en')));
}

function holes(text: string): string {
  return (text.match(/\$\d/g) ?? []).toSorted().join();
}

test('there are the same messages in every language', () => {
  assert.deepEqual(
    failing((own, english) => Object.keys(own).join() === Object.keys(english).join()),
    [],
  );
});

test('there are the same holes in messages in every language', () => {
  assert.deepEqual(
    failing((own, english) =>
      Object.keys(english).every(
        (key) => holes(own[key]?.message ?? '') === holes(english[key]?.message ?? ''),
      ),
    ),
    [],
  );
});
