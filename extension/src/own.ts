import { browser } from './browser.ts';
import type { Messages, Speech } from './speech.ts';

/** The language of the browser. */
export const browsers: Speech = {
  text: (key, values) => browser.i18n.getMessage(key, values),
};

/** The messages in the file of the extension itself. */
export async function messages(path: string): Promise<Messages> {
  return (await fetch(browser.runtime.getURL(path))).json();
}
