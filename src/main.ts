import type { Bridge } from './shell.ts';
import { Shell } from './shell.ts';
import { start } from './start.ts';

declare global {
  interface Window {
    /**
     * The bridge Tauri adds to every window of its own. The package @tauri-apps/api calls it too;
     * calling it directly does without the 41 KB script that window.__TAURI__ needs.
     */
    readonly __TAURI_INTERNALS__: Bridge;
  }
}

start(new Shell(window.__TAURI_INTERNALS__), document.body);
