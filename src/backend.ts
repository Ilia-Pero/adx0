import type { Words } from './words.ts';

/** A list in the window. */
export interface Row {
  readonly id: string;
  readonly title: string;
  readonly note: string;
  readonly on: boolean;
  /** When it was downloaded, in seconds since 1970. Zero if it is not downloaded yet. */
  readonly updated: number;
}

/** A language of the program. */
export interface Tongue {
  readonly code: string;
  readonly title: string;
}

/** What a person has chosen. */
export interface Settings {
  readonly sources: ReadonlyArray<string>;
  readonly blocked: ReadonlyArray<string>;
  readonly allowed: ReadonlyArray<string>;
  /** The code of the language. Empty means the language of the system. */
  readonly language: string;
}

/** Everything the window shows. */
export interface Report {
  /** How many addresses are blocked now. Zero if blocking is off. */
  readonly count: number;
  readonly rows: ReadonlyArray<Row>;
  readonly settings: Settings;
  /** The code of the current language. */
  readonly spoken: string;
  readonly languages: ReadonlyArray<Tongue>;
  readonly words: Words;
}

/** The part of the program that works with files and the internet. */
export interface Backend {
  state(): Promise<Report>;
  save(settings: Settings): Promise<void>;
  /** Downloads the chosen lists. Returns messages about those that could not be downloaded. */
  refresh(): Promise<ReadonlyArray<string>>;
  /** Turns blocking on or off. */
  turn(on: boolean): Promise<void>;
}
