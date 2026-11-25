import type { Backend, Report, Settings } from './backend.ts';
import { LANGUAGES, code, spoken } from './languages.ts';
import { filled } from './words.ts';

const NOW = Math.floor(Date.now() / 1000);

const SOURCES = [
  { id: 'yoyo', title: 'Peter Lowe', note: 'source.yoyo' },
  { id: 'adaway', title: 'AdAway', note: 'source.adaway' },
  { id: 'ruadlist', title: 'RU AdList', note: 'source.ruadlist' },
  { id: 'listefr', title: 'Liste FR', note: 'source.listefr' },
  { id: 'easylistchina', title: 'EasyList China', note: 'source.easylistchina' },
  { id: 'stevenblack', title: 'StevenBlack', note: 'source.stevenblack' },
] as const;

function later<T>(value: T): Promise<T> {
  return new Promise((resolve) => {
    window.setTimeout(() => resolve(value), 400);
  });
}

/**
 * A browser tab in place of the program: it remembers the choice until the page is reloaded,
 * downloads nothing and does not touch the hosts file. For checking the window without the shell.
 */
export class Sandbox implements Backend {
  private readonly tag: string;
  private settings: Settings = {
    sources: ['yoyo', 'adaway'],
    blocked: [],
    allowed: [],
    language: '',
    fresh: true,
  };
  private downloaded: ReadonlyArray<string> = [];
  private count = 0;

  /** The tag is the language of the system. */
  constructor(tag: string) {
    this.tag = tag;
  }

  state(): Promise<Report> {
    const words = this.words();
    return later({
      count: this.count,
      rows: SOURCES.map((source) => ({
        id: source.id,
        title: source.title,
        note: words[source.note],
        on: this.settings.sources.indexOf(source.id) >= 0,
        updated: this.downloaded.indexOf(source.id) >= 0 ? NOW : 0,
      })),
      settings: this.settings,
      spoken: code(this.settings.language || this.tag),
      languages: Object.keys(LANGUAGES).map((name) => ({
        code: name,
        title: spoken(name).language,
      })),
      words,
    });
  }

  save(settings: Settings): Promise<void> {
    this.settings = settings;
    return later(void 0);
  }

  refresh(): Promise<ReadonlyArray<string>> {
    const failed: ReadonlyArray<string> = this.settings.sources.filter(
      (id) => id === 'stevenblack',
    );
    this.downloaded = this.settings.sources.filter((id) => failed.indexOf(id) < 0);
    return later(
      failed.map((id) =>
        filled(filled(this.words()['error.silent'], 'url', id), 'reason', 'timeout'),
      ),
    );
  }

  turn(on: boolean): Promise<void> {
    this.count = on ? 10086 : 0;
    return later(void 0);
  }

  private words(): ReturnType<typeof spoken> {
    return spoken(this.settings.language || this.tag);
  }
}
