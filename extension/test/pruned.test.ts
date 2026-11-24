import assert from 'node:assert/strict';
import { test } from 'node:test';
import { pruned } from '../src/pruned.ts';

test('the schedule of ads disappears from an answer of YouTube', () => {
  assert.deepEqual(pruned({ adPlacements: [1], playerAds: [2], videoDetails: { title: 'A' } }), {
    videoDetails: { title: 'A' },
  });
});

test('the schedule disappears from an answer inside an answer', () => {
  assert.deepEqual(pruned({ playerResponse: { adSlots: [1], streamingData: {} } }), {
    playerResponse: { streamingData: {} },
  });
});

test('another answer does not change', () => {
  assert.deepEqual(pruned({ items: [{ id: 1 }] }), { items: [{ id: 1 }] });
});

test('an answer that is not an object does not change', () => {
  assert.equal(pruned('text'), 'text');
});
