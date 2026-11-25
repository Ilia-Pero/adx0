import own from '../../data/own.json' with { type: 'json' };
import type { Message, Sender, Status } from './browser.ts';
import { browser } from './browser.ts';
import { harmless } from './essential.ts';
import { joined } from './filters.ts';
import { browsers, messages } from './own.ts';
import { place, Shelf } from './shelf.ts';
import { suitable } from './lists.ts';
import { NUMBERS, Rules } from './rules.ts';
import { PERIOD, due } from './schedule.ts';
import { Site, sheet } from './site.ts';
import { spoken } from './speech.ts';

/** The name of the alarm that rings when the lists have to be downloaded. */
const ALARM = 'refresh';

const store = browser.storage.local;

/** The shelf with downloaded lists. It tells about failures in the language of the moment. */
async function shelf(): Promise<Shelf> {
  const speech = await spoken(store, browsers, messages);
  return new Shelf({ text: downloaded }, store, {
    silent: (list, reason) => speech.text('silent', [list.title, reason]),
    short: (list, rules) => speech.text('short', [list.title, String(rules)]),
  });
}

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
  const full = await shelf();
  const taken = await Promise.all(lists.map((list) => full.taken(list)));
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

/** Sets the alarm if there is none: Firefox forgets alarms when it is closed, Chromium may do so. */
async function wind(): Promise<void> {
  if ((await browser.alarms.get(ALARM)) === undefined) {
    const updated = number((await store.get(['updated']))['updated']);
    await browser.alarms.create(ALARM, {
      delayInMinutes: due(updated, Date.now()),
      periodInMinutes: PERIOD,
    });
  }
}

browser.runtime.onInstalled.addListener(renew);
browser.alarms.onAlarm.addListener(renew);
// This part starts whenever it is needed: between updates and pages the browser unloads it.
wind().catch(() => 'there will be another start');

/** Whether the message has come from a window of the extension and not from a page. */
function inside(sender: Sender): boolean {
  return sender.url?.startsWith(browser.runtime.getURL('')) ?? false;
}

async function answered(message: Message, sender: Sender): Promise<ReadonlyArray<Status>> {
  if (message.kind !== 'page' && !inside(sender)) {
    throw new Error('A page may not ask for this');
  }
  switch (message.kind) {
    case 'page':
      await dress(sender);
      return [];
    case 'status':
      return [await status(message.host)];
    case 'allow':
      await allow(message.host, message.on);
      return [];
    case 'refresh':
      await refresh();
      return [];
  }
}

// Chromium waits for an answer if it is told that there will be one; it does not wait for promises.
browser.runtime.onMessage.addListener((message, sender, answer) => {
  answered(message, sender)
    .then(
      (found) => answer({ reason: '', status: found }),
      (error: unknown) => answer({ reason: String(error), status: [] }),
    )
    .catch(() => 'the one who asked has gone');
  return true;
});
