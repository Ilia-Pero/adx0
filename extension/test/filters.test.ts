import assert from 'node:assert/strict';
import { test } from 'node:test';
import type { Filters } from '../src/filters.ts';
import { FilterText, count, joined } from '../src/filters.ts';

function hosts(text: string): Filters {
  return new FilterText(text, 'hosts', false).filters();
}

function adblock(text: string): Filters {
  return new FilterText(text, 'adblock', true).filters();
}

test('a hosts line blocks the site everywhere', () => {
  assert.deepEqual(Array.from(hosts('0.0.0.0 ads.example.com').everywhere), ['ads.example.com']);
});

test('a hosts line with a real address is skipped', () => {
  assert.equal(hosts('203.0.113.7 bank.example.com').everywhere.size, 0);
});

test('the name of this computer is skipped', () => {
  assert.equal(hosts('127.0.0.1  localhost').everywhere.size, 0);
});

test('a comment after the address is dropped', () => {
  assert.deepEqual(Array.from(hosts('0.0.0.0 a.com # b.com').everywhere), ['a.com']);
});

test('comments and headings are skipped', () => {
  assert.equal(hosts('# a.com\n! b.com\n[Adblock Plus 2.0]\n').everywhere.size, 0);
});

test('a site in a stray rule is not blocked', () => {
  assert.equal(hosts('youtube.com#?#.banner').everywhere.size, 0);
});

test('a rule without conditions blocks the site everywhere', () => {
  assert.deepEqual(Array.from(adblock('||ads.example.com^').everywhere), ['ads.example.com']);
});

test('a rule for pages of other sites blocks the site only there', () => {
  assert.deepEqual(Array.from(adblock('||example.com^$third-party').foreign), ['example.com']);
});

test('a rule with other conditions is skipped', () => {
  assert.equal(adblock('||example.com^$script,domain=a.com').everywhere.size, 0);
});

test('a rule for a part of a site is skipped', () => {
  assert.equal(adblock('||example.com/banner^').everywhere.size, 0);
});

test('an exception is skipped', () => {
  assert.equal(adblock('@@||example.com^').everywhere.size, 0);
});

test('a name alone among rules does not block the site', () => {
  assert.equal(adblock('[Adblock Plus 2.0]\nyandex.ru').everywhere.size, 0);
});

test('a site in an extended rule is not blocked', () => {
  assert.equal(adblock('youtube.com#?#.banner').everywhere.size, 0);
});

test('a site in a rule with a style is not blocked', () => {
  assert.equal(adblock('ok.ru#$#body { top: 0 }').everywhere.size, 0);
});

test('a selector without a site works on all pages', () => {
  assert.deepEqual(Array.from(adblock('##.banner').generic), ['.banner']);
});

test('a selector with a site works only there', () => {
  assert.deepEqual(Array.from(adblock('a.com,b.com##.banner').sites.get('b.com') ?? []), [
    '.banner',
  ]);
});

test('a selector does not work on the site it excludes', () => {
  assert.deepEqual(Array.from(adblock('~a.com##.banner').spared.get('a.com') ?? []), ['.banner']);
});

test('an exception takes the selector off the site', () => {
  assert.deepEqual(Array.from(adblock('a.com#@#.banner').spared.get('a.com') ?? []), ['.banner']);
});

test('an extended selector is skipped', () => {
  assert.equal(adblock('##.banner:has-text(Ad)').generic.size, 0);
});

test('a script in place of a selector is skipped', () => {
  assert.equal(adblock('a.com##+js(set, ads, false)').sites.size, 0);
});

test('a selector with CSS of its own is skipped', () => {
  assert.equal(adblock('##a{}body{background:red').generic.size, 0);
});

test('selectors are not taken from a list without them', () => {
  assert.equal(new FilterText('##.banner', 'adblock', false).filters().generic.size, 0);
});

test('every kind of rule counts', () => {
  assert.equal(count(adblock('||a.com^\n||b.com^$third-party\n##.banner\nc.com##.ad')), 4);
});

test('several lists add up without repeats', () => {
  assert.deepEqual(
    Array.from(joined([hosts('a.com\nb.com'), adblock('||b.com^')]).everywhere),
    ['a.com', 'b.com'],
  );
});
