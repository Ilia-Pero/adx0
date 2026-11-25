import type { Backend, Release, Report } from './backend.ts';
import { element } from './elements.ts';
import { filled } from './words.ts';

/** The version of the program, and the button that looks for a newer one. */
export class Edition {
  private readonly backend: Backend;
  private readonly note = element('p', 'foot');
  private readonly check = element('button', '');
  private readonly get = element('button', 'hidden');
  private readonly news = element('button', 'hidden');
  private report: Report;

  constructor(backend: Backend, report: Report) {
    this.backend = backend;
    this.report = report;
  }

  /** The part of the window about it. */
  place(): HTMLElement {
    const place = element('div', 'edition');
    this.note.setAttribute('role', 'status');
    this.check.addEventListener('click', () => this.look());
    this.get.addEventListener('click', () => this.run(this.backend.download()));
    this.news.addEventListener('click', () => this.run(this.backend.news()));
    [this.note, this.check, this.get, this.news].forEach((node) => place.appendChild(node));
    return place;
  }

  show(report: Report): void {
    this.report = report;
    this.note.textContent = `ad×0 ${report.version}`;
    this.check.textContent = report.words['update.check'];
    this.get.textContent = report.words['update.get'];
    this.news.textContent = report.words['update.news'];
    this.offer(false);
  }

  private look(): void {
    this.note.textContent = this.report.words['update.checking'];
    this.check.disabled = true;
    this.run(this.backend.update().then((release) => this.tell(release)));
  }

  private tell(release: Release): void {
    this.note.textContent = release.newer
      ? filled(this.report.words['update.found'], 'version', release.version)
      : `ad×0 ${this.report.version}. ${this.report.words['update.latest']}`;
    this.offer(release.newer);
  }

  /** Shows the buttons with the new version instead of the one that looks for it. */
  private offer(newer: boolean): void {
    this.check.classList.toggle('hidden', newer);
    [this.get, this.news].forEach((node) => node.classList.toggle('hidden', !newer));
  }

  private run(work: Promise<void>): void {
    const free = (): void => {
      this.check.disabled = false;
    };
    work
      .catch((error: unknown) => {
        this.note.textContent = String(error);
      })
      .then(free, free);
  }
}
