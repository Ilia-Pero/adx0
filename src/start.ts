import type { Backend } from './backend.ts';
import { Panel } from './panel.ts';

/** Opens the window as soon as the program has told what to show in it. */
export function start(backend: Backend, root: HTMLElement): void {
  backend
    .state()
    .then((report) => new Panel(backend, root, report).open())
    .catch((error: unknown) => {
      root.textContent = String(error);
    });
}
