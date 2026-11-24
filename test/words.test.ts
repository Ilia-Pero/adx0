import assert from 'node:assert/strict';
import { test } from 'node:test';
import { spoken } from '../src/languages.ts';
import { filled } from '../src/words.ts';

test('a system is spoken to in its own language', () => {
  assert.equal(spoken('ru-RU')['turn.on'], 'Включить');
});

test('a tag with a script is understood', () => {
  assert.equal(spoken('zh-Hans-CN').language, '简体中文');
});

test('an unknown language falls back to English', () => {
  assert.equal(spoken('fi-FI')['turn.on'], 'Turn on');
});

test('a language that merely starts with the same letters is not taken for it', () => {
  assert.equal(spoken('run')['turn.on'], 'Turn on');
});

test('a hole in the text is filled', () => {
  assert.equal(filled('Disk error: {reason}', 'reason', 'full'), 'Disk error: full');
});
