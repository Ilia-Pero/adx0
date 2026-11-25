import { Regional } from '../../src/regional.ts';
import type { Status } from './browser.ts';
import { asked, browser } from './browser.ts';
import { browsers, messages } from './own.ts';
import type { Speech } from './speech.ts';
import { TONGUES, choice, choose, spoken } from './speech.ts';

const store = browser.storage.local;

function part<T extends HTMLElement>(id: string, kind: new () => T): T {
  const node = document.getElementById(id);
  if (node instanceof kind) {
    return node;
  }
  throw new Error(`The popup has no part ${id}`);
}

const tongues = part('speech', HTMLSelectElement);

/** The site in the active tab. An empty text if it is a page of the browser itself. */
async function host(): Promise<string> {
  const tabs = await browser.tabs.query({ active: true, currentWindow: true });
  const url = tabs[0]?.url ?? '';
  return /^https?:/.test(url) ? new URL(url).hostname : '';
}

async function status(site: string): Promise<Status> {
  const [found] = await asked({ kind: 'status', host: site });
  if (found === undefined) {
    throw new Error('The extension does not answer');
  }
  return found;
}

function lists(state: Status, speech: Speech): string {
  const written = new Regional(speech.text('code', []));
  return state.updated === 0
    ? speech.text('never', [])
    : speech.text('lists', [written.number(state.addresses), written.day(state.updated)]);
}

/** Fills the list of languages. */
async function offer(speech: Speech): Promise<void> {
  tongues.title = speech.text('speech', []);
  tongues.setAttribute('aria-label', tongues.title);
  tongues.textContent = '';
  [{ code: '', title: speech.text('auto', []) }, ...TONGUES].forEach((tongue) => {
    const option = document.createElement('option');
    option.value = tongue.code;
    option.textContent = tongue.title;
    tongues.appendChild(option);
  });
  tongues.value = await choice(store);
}

async function show(): Promise<void> {
  const speech = await spoken(store, browsers, messages);
  const code = speech.text('code', []);
  const site = await host();
  const state = await status(site);
  document.documentElement.lang = code;
  document.documentElement.dir = code === 'he' ? 'rtl' : 'ltr';
  part('refresh', HTMLButtonElement).textContent = speech.text('update', []);
  part('sign', HTMLElement).textContent = state.allowed ? 'ad × 1 = ad' : 'ad × 0 = 0';
  part('site', HTMLElement).textContent = site === '' ? speech.text('service', []) : site;
  part('state', HTMLElement).textContent = speech.text(state.allowed ? 'allowed' : 'blocked', []);
  part('allow', HTMLElement).textContent = speech.text(state.allowed ? 'block' : 'allow', []);
  part('allow', HTMLElement).hidden = site === '';
  part('lists', HTMLElement).textContent = lists(state, speech);
  part('alarm', HTMLElement).textContent = state.failures.join(' ');
  part('alarm', HTMLElement).hidden = state.failures.length === 0;
  document.body.classList.toggle('open', state.allowed);
  await offer(speech);
}

async function toggle(): Promise<void> {
  const site = await host();
  const state = await status(site);
  await asked({ kind: 'allow', host: site, on: !state.allowed });
  const tabs = await browser.tabs.query({ active: true, currentWindow: true });
  const id = tabs[0]?.id;
  if (id !== undefined) {
    await browser.tabs.reload(id);
  }
  await show();
}

async function refresh(): Promise<void> {
  const speech = await spoken(store, browsers, messages);
  part('lists', HTMLElement).textContent = speech.text('loading', []);
  await asked({ kind: 'refresh' });
  await show();
}

function complain(error: unknown): void {
  part('alarm', HTMLElement).textContent = String(error);
  part('alarm', HTMLElement).hidden = false;
}

part('allow', HTMLButtonElement).addEventListener('click', () => {
  toggle().catch(complain);
});
part('refresh', HTMLButtonElement).addEventListener('click', () => {
  refresh().catch(complain);
});
tongues.addEventListener('change', () => {
  choose(store, tongues.value).then(show).catch(complain);
});
show().catch(complain);
