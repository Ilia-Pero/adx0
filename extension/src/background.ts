import own from '../../data/own.json' with { type: 'json' };
import type { Message, Sender, Status } from './browser.ts';
import { harmless } from './essential.ts';
import { joined } from './filters.ts';
import { place, Shelf } from './shelf.ts';
import { suitable } from './lists.ts';
import { NUMBERS, Rules } from './rules.ts';
import { Site, sheet } from './site.ts';

/** Once a day. Between updates and pages the browser unloads this part of the extension. */
const PERIOD = 24 * 60;

const store = browser.storage.local;

const shelf = new Shelf({ text: downloaded }, store, {
  silent: (list, reason) => browser.i18n.getMessage('silent', [list.title, reason]),
  short: (list, rules) => browser.i18n.getMessage('short', [list.title, String(rules)]),
});

function strings(value: unknown): ReadonlyArray<string> {
  return Array.isArray(value) ? value.filter((item) => typeof item === 'string') : [];
}

function number(value: unknown): number {
  return typeof value === 'number' ? value : 0;
}

async function downloaded(url: string): Promise<string> {
  const answer = await fetch(url, { cache: 'no-cache', credentials: 'omit' });
  if (!answer.ok) {
    throw new Error(String(answer.status));
  }
  return answer.text();
}

/** Languages a person reads. */
async function languages(): Promise<ReadonlyArray<string>> {
  return [browser.i18n.getUILanguage(), ...(await browser.i18n.getAcceptLanguages())];
}

/** Gives the browser the rules that follow from what is kept. */
async function enforce(): Promise<void> {
  const kept = await store.get(['everywhere', 'foreign', 'allowed']);
  await browser.declarativeNetRequest.updateDynamicRules({
    removeRuleIds: NUMBERS,
    addRules: new Rules(
      strings(kept['everywhere']),
      strings(kept['foreign']),
      strings(kept['allowed']),
    ).list(),
  });
}

/** Downloads the lists and replaces the previous ones with them. */
async function refresh(): Promise<void> {
  const lists = suitable(await languages());
  const taken = await Promise.all(lists.map((list) => shelf.taken(list)));
  const before = strings((await store.get(['keys']))['keys']);
  const filters = joined(taken.map((one) => one.filters));
  const values: Record<string, unknown> = {};
  filters.sites.forEach((selectors, name) => {
    values[`site:${name}`] = Array.from(selectors);
  });
  filters.spared.forEach((selectors, name) => {
    values[`spare:${name}`] = Array.from(selectors);
  });
  const keys = new Set([...Object.keys(values), ...lists.map(place)]);
  // The new first, cleaning after it: if writing fails, the previous lists stay whole.
  await store.set({
    ...values,
    keys: Array.from(keys),
    everywhere: [...harmless(filters.everywhere), ...own],
    foreign: harmless(filters.foreign),
    generic: Array.from(filters.generic),
    ...(taken.some((one) => one.failures.length === 0) ? { updated: Date.now() } : {}),
    failures: taken.flatMap((one) => one.failures),
  });
  await store.remove(before.filter((key) => !keys.has(key)));
  await enforce();
}

/** Hides ads on the page that has just opened. */
async function dress(sender: Sender): Promise<void> {
  const tab = sender.tab?.id;
  if (tab === undefined || sender.url === undefined) {
    return;
  }
  const names = new Site(new URL(sender.url).hostname).names();
  const top = new Site(new URL(sender.tab?.url ?? sender.url).hostname);
  const kept = await store.get([
    'allowed',
    'generic',
    ...names.map((name) => `site:${name}`),
    ...names.map((name) => `spare:${name}`),
  ]);
  if (top.within(strings(kept['allowed']))) {
    return;
  }
  const css = sheet(
    [...strings(kept['generic']), ...names.flatMap((name) => strings(kept[`site:${name}`]))],
    names.flatMap((name) => strings(kept[`spare:${name}`])),
  );
  if (css !== '') {
    await browser.scripting.insertCSS({
      target: { tabId: tab, frameIds: [sender.frameId ?? 0] },
      css,
      origin: 'USER',
    });
  }
}

async function status(host: string): Promise<Status> {
  const kept = await store.get(['allowed', 'everywhere', 'foreign', 'updated', 'failures']);
  return {
    allowed: new Site(host).within(strings(kept['allowed'])),
    addresses: strings(kept['everywhere']).length + strings(kept['foreign']).length,
    updated: number(kept['updated']),
    failures: strings(kept['failures']),
  };
}

/** Allows ads on the site or blocks them again. */
async function allow(host: string, on: boolean): Promise<void> {
  const names = new Site(host).names();
  const kept = strings((await store.get(['allowed']))['allowed']);
  const rest = kept.filter((name) => !names.includes(name));
  await store.set({ allowed: on ? [...rest, ...names.slice(0, 1)] : rest });
  await enforce();
}

function renew(): void {
  refresh().catch((error: unknown) => store.set({ failures: [String(error)] }));
}

browser.runtime.onInstalled.addListener(() => {
  browser.alarms.create('refresh', { delayInMinutes: PERIOD, periodInMinutes: PERIOD });
  renew();
});

browser.alarms.onAlarm.addListener(renew);

browser.runtime.onMessage.addListener((message: Message, sender: Sender): Promise<unknown> => {
  switch (message.kind) {
    case 'page':
      return dress(sender);
    case 'status':
      return status(message.host);
    case 'allow':
      return allow(message.host, message.on);
    case 'refresh':
      return refresh();
  }
});
