import assert from 'node:assert/strict';
import { test } from 'node:test';
import { PERIOD, due } from '../src/schedule.ts';

const HOUR = 3_600_000;
const NOW = 1_000 * HOUR;

test('lists are downloaded when they are a day old', () => {
  assert.equal(due(NOW - 20 * HOUR, NOW), 4 * 60);
});

test('overdue lists are downloaded in a minute', () => {
  assert.equal(due(NOW - 90 * HOUR, NOW), 1);
});

test('lists that are being downloaded for the first time wait for a day', () => {
  assert.equal(due(0, NOW), PERIOD);
});

test('lists wait for no more than a day when the clock has been moved', () => {
  assert.equal(due(NOW + 500 * HOUR, NOW), PERIOD);
});
