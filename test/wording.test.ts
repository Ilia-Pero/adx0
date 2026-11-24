import assert from 'node:assert/strict';
import { test } from 'node:test';
import { age, formula, summary } from '../src/wording.ts';
import { report, row } from './fakes.ts';

/** Noon of 2026-11-23 in Greenwich: the same day almost everywhere. */
const MONDAY = 1795435200;

test('while blocking is on, an ad is multiplied by zero', () => {
  assert.equal(formula(report('en', 10086, [])), 'ad × 0 = 0');
});

test('while blocking is off, an ad is multiplied by one', () => {
  assert.equal(formula(report('en', 0, [])), 'ad × 1 = ad');
});

test('while blocking is off, the window offers to turn it on', () => {
  assert.match(summary(report('en', 0, [])), /^Turn it on/);
});

test('while blocking is on, the window says how much is blocked', () => {
  assert.match(summary(report('en', 10086, [])), /^Addresses blocked: 10,086\./);
});

test('the number is written the way the language writes numbers', () => {
  assert.match(summary(report('de', 10086, [])), /10\.086/);
});

test('the window names the day of the oldest list', () => {
  assert.match(
    summary(report('en', 1, [row('yoyo', true, MONDAY + 864000), row('adaway', true, MONDAY)])),
    /November 23, 2026/,
  );
});

test('a list that is off has no say in the day', () => {
  assert.doesNotMatch(
    summary(report('en', 1, [row('yoyo', true, MONDAY), row('adaway', false, MONDAY - 864000)])),
    /November 13/,
  );
});

test('the day is written in the language of the window', () => {
  assert.match(summary(report('ru', 1, [row('yoyo', true, MONDAY)])), /23 ноября 2026/);
});

test('a date with a dot of its own does not end with two dots', () => {
  assert.doesNotMatch(summary(report('ru', 1, [row('yoyo', true, MONDAY)])), /\.\./);
});

test('there are no spaces between sentences in Japanese', () => {
  assert.doesNotMatch(summary(report('ja', 1, [row('yoyo', true, MONDAY)])), / /);
});

test('a list that was never downloaded says so', () => {
  assert.equal(age(row('yoyo', true, 0), report('en', 0, [])), 'not downloaded yet');
});

test('a downloaded list names its day', () => {
  assert.equal(
    age(row('yoyo', true, MONDAY), report('en', 0, [])),
    'downloaded on November 23, 2026',
  );
});
