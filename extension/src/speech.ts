import type { Store } from './shelf.ts';

/** Texts of the extension in one language. */
export interface Speech {
  /** The text under the key, with values in place of $1 and $2. */
  text(key: string, values: ReadonlyArray<string>): string;
}

/** Messages of one language as they lie in _locales. */
export type Messages = Readonly<Record<string, { readonly message: string }>>;

/** A language a person can choose. An empty code means the language of the browser. */
export interface Tongue {
  readonly code: string;
  readonly title: string;
}

/** Languages of the extension, each in its own language. The codes are folders in _locales. */
export const TONGUES: ReadonlyArray<Tongue> = [
  { code: 'de', title: 'Deutsch' },
  { code: 'el', title: 'Ελληνικά' },
  { code: 'en', title: 'English' },
  { code: 'es', title: 'Español' },
  { code: 'fr', title: 'Français' },
  { code: 'he', title: 'עברית' },
  { code: 'it', title: 'Italiano' },
  { code: 'ja', title: '日本語' },
  { code: 'ko', title: '한국어' },
  { code: 'ru', title: 'Русский' },
  { code: 'sr', title: 'Српски' },
  { code: 'zh_CN', title: '简体中文' },
];

/** A language chosen by a person. A key it has no text for is shown as it is. */
export class Chosen implements Speech {
  private readonly messages: Messages;

  constructor(messages: Messages) {
    this.messages = messages;
  }

  text(key: string, values: ReadonlyArray<string>): string {
    const message = this.messages[key]?.message ?? key;
    return message.replace(/\$(\d)/g, (hole, number: string) => values[Number(number) - 1] ?? hole);
  }
}

/** The code of the chosen language. Empty if there is no choice, or it makes no sense. */
export async function choice(store: Store): Promise<string> {
  const kept = (await store.get(['language']))['language'];
  return TONGUES.map((tongue) => tongue.code).find((code) => code === kept) ?? '';
}

/** Remembers the choice. An empty code returns it to the browser. */
export function choose(store: Store, code: string): Promise<void> {
  return store.set({ language: code });
}

/** The chosen language; the one of the browser if there is no choice. */
export async function spoken(
  store: Store,
  browsers: Speech,
  read: (path: string) => Promise<Messages>,
): Promise<Speech> {
  const code = await choice(store);
  return code === '' ? browsers : new Chosen(await read(`_locales/${code}/messages.json`));
}
