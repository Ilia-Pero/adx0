import assert from 'node:assert/strict';
import { test } from 'node:test';
import { harmless } from '../src/essential.ts';
import { NUMBERS, Rules } from '../src/rules.ts';

test('an essential site stays open even if a list is wrong', () => {
  assert.deepEqual(harmless(['youtube.com', 'www.yandex.ru', 'ads.youtube.com']), [
    'ads.youtube.com',
  ]);
});

test('there are no rules without addresses', () => {
  assert.deepEqual(new Rules([], [], []).list(), []);
});

test('all addresses fit into one rule', () => {
  assert.deepEqual(new Rules(['a.com', 'b.com'], [], []).list()[0]?.condition.requestDomains, [
    'a.com',
    'b.com',
  ]);
});

test('the page of an ad site itself can be opened', () => {
  assert.deepEqual(new Rules(['a.com'], [], []).list()[0]?.condition.excludedResourceTypes, [
    'main_frame',
  ]);
});

test('an allowed site is stronger than blocked addresses', () => {
  assert.equal(new Rules([], [], ['a.com']).list()[0]?.priority, 10);
});

test('every rule has a known number', () => {
  assert.deepEqual(
    new Rules(['a.com'], ['b.com'], ['c.com']).list().map((rule) => rule.id),
    NUMBERS,
  );
});

test('the ad script of Yandex is blocked together with ad addresses', () => {
  assert.equal(new Rules(['a.com'], [], []).list()[1]?.condition.urlFilter, '/ads/system/');
});
