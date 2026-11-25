import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Regional } from '../src/regional.ts';

/** Noon of 2026-11-23 in Greenwich: the same day almost everywhere. */
const MONDAY = 1795435200000;

test('a day is written in the language', () => {
  assert.equal(new Regional('en').day(MONDAY), 'November 23, 2026');
});

test('a number is written the way the language writes numbers', () => {
  assert.equal(new Regional('de').number(10086), '10.086');
});

test('a day in a language the browser does not know is written with digits', () => {
  assert.equal(new Regional('xx').day(MONDAY), '23.11.2026');
});

test('a number in a language the browser does not know has spaces', () => {
  assert.match(new Regional('xx').number(10086), /^10\s086$/);
});
