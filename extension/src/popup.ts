import type { Status } from './browser.ts';

/** The text in the language of the browser. */
function text(key: string, values: ReadonlyArray<string>): string {
  return browser.i18n.getMessage(key, values);
}

function part(id: string): HTMLElement {
  const node = document.getElementById(id);
  if (node === null) {
    throw new Error(`The popup has no part ${id}`);
  }
  return node;
}

/** The site in the active tab. An empty text if it is a page of the browser itself. */
async function host(): Promise<string> {
  const tabs = await browser.tabs.query({ active: true, currentWindow: true });
  const url = tabs[0]?.url ?? '';
  return /^https?:/.test(url) ? new URL(url).hostname : '';
}

function day(moment: number): string {
  return new Date(moment).toLocaleDateString(text('code', []), { day: 'numeric', month: 'long' });
}

function lists(state: Status): string {
  return state.updated === 0
    ? text('never', [])
    : text('lists', [state.addresses.toLocaleString(text('code', [])), day(state.updated)]);
}

async function show(): Promise<void> {
  const site = await host();
  const state = await browser.runtime.sendMessage<Status>({ kind: 'status', host: site });
  part('sign').textContent = state.allowed ? 'ad × 1 = ad' : 'ad × 0 = 0';
  part('site').textContent = site === '' ? text('service', []) : site;
  part('state').textContent = text(state.allowed ? 'allowed' : 'blocked', []);
  part('allow').textContent = text(state.allowed ? 'block' : 'allow', []);
  part('allow').hidden = site === '';
  part('lists').textContent = lists(state);
  part('alarm').textContent = state.failures.join(' ');
  part('alarm').hidden = state.failures.length === 0;
  document.body.classList.toggle('open', state.allowed);
}

async function toggle(): Promise<void> {
  const site = await host();
  const state = await browser.runtime.sendMessage<Status>({ kind: 'status', host: site });
  await browser.runtime.sendMessage<void>({ kind: 'allow', host: site, on: !state.allowed });
  const tabs = await browser.tabs.query({ active: true, currentWindow: true });
  const id = tabs[0]?.id;
  if (id !== undefined) {
    await browser.tabs.reload(id);
  }
  await show();
}

async function refresh(): Promise<void> {
  part('lists').textContent = text('loading', []);
  await browser.runtime.sendMessage<void>({ kind: 'refresh' });
  await show();
}

function complain(error: unknown): void {
  part('alarm').textContent = String(error);
  part('alarm').hidden = false;
}

document.documentElement.lang = text('code', []);
document.documentElement.dir = text('code', []) === 'he' ? 'rtl' : 'ltr';
part('refresh').textContent = text('update', []);
part('allow').addEventListener('click', () => {
  toggle().catch(complain);
});
part('refresh').addEventListener('click', () => {
  refresh().catch(complain);
});
show().catch(complain);
