import assert from 'node:assert/strict';
import { test } from 'node:test';
import { Site, sheet } from '../src/site.ts';

test('a site has names of all sites it is a part of', () => {
  assert.deepEqual(new Site('a.b.example.com').names(), [
    'a.b.example.com',
    'b.example.com',
    'example.com',
  ]);
});

test('www at the beginning is not a part of the name', () => {
  assert.deepEqual(new Site('www.example.com').names(), ['example.com']);
});

test('a subdomain is in the list together with its site', () => {
  assert.equal(new Site('news.example.com').within(['example.com']), true);
});

test('a lookalike is not in the list', () => {
  assert.equal(new Site('badexample.com').within(['example.com']), false);
});

test('every selector has a rule of its own', () => {
  assert.equal(
    sheet(['.a', '.b'], []),
    '.a{display:none!important}\n.b{display:none!important}',
  );
});

test('a selector taken off the site does not work', () => {
  assert.equal(sheet(['.a', '.b'], ['.a']), '.b{display:none!important}');
});
