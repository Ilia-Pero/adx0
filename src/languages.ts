import de from '../locales/de.json' with { type: 'json' };
import el from '../locales/el.json' with { type: 'json' };
import en from '../locales/en.json' with { type: 'json' };
import es from '../locales/es.json' with { type: 'json' };
import fr from '../locales/fr.json' with { type: 'json' };
import he from '../locales/he.json' with { type: 'json' };
import it from '../locales/it.json' with { type: 'json' };
import ja from '../locales/ja.json' with { type: 'json' };
import ko from '../locales/ko.json' with { type: 'json' };
import ru from '../locales/ru.json' with { type: 'json' };
import sr from '../locales/sr.json' with { type: 'json' };
import zh from '../locales/zh.json' with { type: 'json' };
import type { Words } from './words.ts';

/** All languages by their codes. For the demo page and tests. */
export const LANGUAGES: Readonly<Record<string, Words>> = {
  en,
  de,
  el,
  es,
  fr,
  he,
  it,
  ja,
  ko,
  ru,
  sr,
  zh,
};

/** The code of the language with the tag, such as ru-RU. English if there is no such language. */
export function code(tag: string): string {
  const short = tag.toLowerCase().replace(/[-_.].*$/, '');
  return Object.keys(LANGUAGES).indexOf(short) < 0 ? 'en' : short;
}

/** Words in the language with the tag. */
export function spoken(tag: string): Words {
  const words = LANGUAGES[code(tag)];
  return words === undefined ? en : words;
}
