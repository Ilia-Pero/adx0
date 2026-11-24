import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Lines } from '../src/lines.ts';

test('every line becomes an entry', () => {
  assert.deepEqual(new Lines('a.com\nb.com').list(), ['a.com', 'b.com']);
});

test('empty lines and spaces around are dropped', () => {
  assert.deepEqual(new Lines('\n  a.com  \r\n\n').list(), ['a.com']);
});

test('repeats are dropped', () => {
  assert.deepEqual(new Lines('a.com\na.com').list(), ['a.com']);
});

test('addresses separated with commas are read one by one', () => {
  assert.deepEqual(new Lines('a.com, b.com').list(), ['a.com', 'b.com']);
});

test('an empty field has no entries', () => {
  assert.deepEqual(new Lines('').list(), []);
});
