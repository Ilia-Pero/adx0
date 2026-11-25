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
      addListener(listen: (message: Message, sender: Sender) => Promise<unknown>): void;
    };
    sendMessage<T>(message: Message): Promise<T>;
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

/** What the popup shows. */
export interface Status {
  readonly allowed: boolean;
  readonly addresses: number;
  /** When the lists were downloaded, in milliseconds since 1970. Zero if they are not yet. */
  readonly updated: number;
  readonly failures: ReadonlyArray<string>;
}

declare global {
  const browser: Browser;
}
