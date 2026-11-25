import type { Backend, Release, Report, Settings } from './backend.ts';

/** The bridge to the Tauri shell: calls commands written in Rust. */
export interface Bridge {
  invoke<T>(command: string, payload?: Readonly<Record<string, unknown>>): Promise<T>;
}

/** The real program: the shell written in Rust works with files and the internet. */
export class Shell implements Backend {
  private readonly bridge: Bridge;

  constructor(bridge: Bridge) {
    this.bridge = bridge;
  }

  state(): Promise<Report> {
    return this.bridge.invoke<Report>('state');
  }

  save(settings: Settings): Promise<void> {
    return this.bridge.invoke<void>('save', { settings });
  }

  refresh(): Promise<ReadonlyArray<string>> {
    return this.bridge.invoke<ReadonlyArray<string>>('refresh');
  }

  turn(on: boolean): Promise<void> {
    return this.bridge.invoke<void>('turn', { on });
  }

  update(): Promise<Release> {
    return this.bridge.invoke<Release>('update');
  }

  download(): Promise<void> {
    return this.bridge.invoke<void>('download');
  }

  news(): Promise<void> {
    return this.bridge.invoke<void>('news');
  }
}
