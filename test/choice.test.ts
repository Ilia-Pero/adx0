import assert from 'node:assert/strict';
import { test } from 'node:test';
import { alike, saved, translated } from '../src/choice.ts';
import { report, row } from './fakes.ts';

const CHOICE = saved(report('en', 0, [row('yoyo', true, 0), row('adaway', false, 0)]));

test('lists that are on are the saved ones', () => {
  assert.deepEqual(CHOICE.sources, ['yoyo']);
});

test('a choice is like itself', () => {
  assert.equal(alike(CHOICE, { ...CHOICE }), true);
});

test('a choice with one more list is another choice', () => {
  assert.equal(alike(CHOICE, { ...CHOICE, sources: ['yoyo', 'adaway'] }), false);
});

test('a choice with another blocked site is another choice', () => {
  assert.equal(alike(CHOICE, { ...CHOICE, blocked: ['c.com'] }), false);
});

test('a choice with another allowed site is another choice', () => {
  assert.equal(alike(CHOICE, { ...CHOICE, allowed: [] }), false);
});

test('a choice without fresh lists is another choice', () => {
  assert.equal(alike(CHOICE, { ...CHOICE, fresh: false }), false);
});

test('a choice in another language blocks the same', () => {
  assert.equal(alike(CHOICE, translated(CHOICE, 'el')), true);
});

test('a choice in another language remembers the language', () => {
  assert.equal(translated(CHOICE, 'el').language, 'el');
});
