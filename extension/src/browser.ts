import type { Rule } from './rules.ts';
import type { Store } from './shelf.ts';

/** A tab of the browser. */
export interface Tab {
  readonly id?: number;
  readonly url?: string;
}

/** Where a message has come from. */
export interface Sender {
  readonly tab?: Tab;
  readonly frameId?: number;
  readonly url?: string;
}

/** The part of the browser the extension uses. */
interface Browser {
  readonly alarms: {
    create(name: string, info: { readonly delayInMinutes: number; readonly periodInMinutes: number }): void;
    readonly onAlarm: { addListener(listen: () => void): void };
  };
  readonly declarativeNetRequest: {
    updateDynamicRules(change: {
      readonly removeRuleIds: ReadonlyArray<number>;
      readonly addRules: ReadonlyArray<Rule>;
    }): Promise<void>;
  };
  readonly i18n: {
    getMessage(key: string, values?: ReadonlyArray<string>): string;
    getAcceptLanguages(): Promise<ReadonlyArray<string>>;
    getUILanguage(): string;
  };
  readonly runtime: {
    getURL(path: string): string;
    readonly onInstalled: { addListener(listen: () => void): void };
    readonly onMessage: {
      addListener(
        listen: (message: Message, sender: Sender, answer: (reply: Reply) => void) => boolean,
      ): void;
    };
    sendMessage(message: Message): Promise<Reply>;
  };
  readonly scripting: {
    insertCSS(injection: {
      readonly target: { readonly tabId: number; readonly frameIds: ReadonlyArray<number> };
      readonly css: string;
      readonly origin: 'USER';
    }): Promise<void>;
  };
  readonly storage: { readonly local: Store };
  readonly tabs: {
    query(what: { readonly active: boolean; readonly currentWindow: boolean }): Promise<ReadonlyArray<Tab>>;
    reload(id: number): Promise<void>;
  };
}

/** Messages inside the extension. */
export type Message =
  | { readonly kind: 'page' }
  | { readonly kind: 'status'; readonly host: string }
  | { readonly kind: 'allow'; readonly host: string; readonly on: boolean }
  | { readonly kind: 'refresh' };

/** An answer to a message. An empty reason means that there is no failure. */
export interface Reply {
  readonly reason: string;
  /** What the popup shows, if this is what was asked for. */
  readonly status: ReadonlyArray<Status>;
}

/** What the popup shows. */
export interface Status {
  readonly allowed: boolean;
  readonly addresses: number;
  /** When the lists were downloaded, in milliseconds since 1970. Zero if they are not yet. */
  readonly updated: number;
  readonly failures: ReadonlyArray<string>;
}

declare global {
  var browser: Browser | undefined;
  var chrome: Browser | undefined;
}

/** Firefox has the one name, Chromium the other; newer ones have both. */
function found(): Browser {
  const known = globalThis.browser ?? globalThis.chrome;
  if (known === undefined) {
    throw new Error('This is not a browser');
  }
  return known;
}

export const browser = found();

/** What the background part of the extension answers. */
export async function asked(message: Message): Promise<ReadonlyArray<Status>> {
  const reply = await browser.runtime.sendMessage(message);
  if (reply.reason !== '') {
    throw new Error(reply.reason);
  }
  return reply.status;
}
