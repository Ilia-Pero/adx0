import type { Backend, Report, Row, Settings } from './backend.ts';
import { alike, saved, translated } from './choice.ts';
import { element, written } from './elements.ts';
import { Lines } from './lines.ts';
import { age, formula, summary } from './wording.ts';
import type { Words } from './words.ts';

type Key = keyof Words;

const GLOBE =
  '<svg viewBox="0 0 20 20" width="18" height="18" fill="none" stroke="currentColor" ' +
  'stroke-width="1.5" aria-hidden="true"><circle cx="10" cy="10" r="7.5"/><path d="M2.5 10h15"/>' +
  '<path d="M10 2.5c2.6 2.2 3.7 4.7 3.7 7.5s-1.1 5.3-3.7 7.5"/>' +
  '<path d="M10 2.5C7.4 4.7 6.3 7.2 6.3 10s1.1 5.3 3.7 7.5"/></svg>';

/** An element and the key of the text in it. */
interface Label {
  readonly node: HTMLElement;
  readonly key: Key;
}

/**
 * The window of the program. The only place where state changes: the last report, what a person
 * has typed into the form, and whether the program is busy.
 */
export class Panel {
  private readonly backend: Backend;
  private readonly root: HTMLElement;
  private readonly sign = element('p', 'sign');
  private readonly title = element('h1', '');
  private readonly note = element('p', 'note');
  private readonly power = element('button', 'main');
  private readonly update = element('button', '');
  private readonly alarm = element('p', 'alarm hidden');
  private readonly sources = element('div', 'sources');
  private readonly allowed = element('textarea', '');
  private readonly blocked = element('textarea', '');
  private readonly apply = element('button', 'main apply hidden');
  private readonly speech = element('select', '');
  private readonly labels: Label[] = [];
  private report: Report;

  constructor(backend: Backend, root: HTMLElement, report: Report) {
    this.backend = backend;
    this.root = root;
    this.report = report;
  }

  open(): void {
    this.build();
    this.fill(saved(this.report));
    this.show(this.report);
    this.power.addEventListener('click', () => {
      const on = this.report.count === 0;
      this.run(on ? 'doing.on' : 'doing.off', () =>
        on ? this.enable() : this.backend.turn(false),
      );
    });
    this.update.addEventListener('click', () => this.run('doing.update', () => this.renew()));
    this.apply.addEventListener('click', () =>
      this.run('doing.apply', () => (this.report.count === 0 ? this.store() : this.enable())),
    );
    this.speech.addEventListener('change', () =>
      this.run('doing.look', () =>
        this.backend.save(translated(saved(this.report), this.speech.value)),
      ),
    );
    this.root.addEventListener('input', () => this.mark());
  }

  private build(): void {
    const head = element('section', 'head');
    const speech = element('label', 'speech');
    speech.innerHTML = GLOBE;
    speech.appendChild(this.speech);
    [speech, this.sign, this.title, this.note, this.power, this.update].forEach((node) =>
      head.appendChild(node),
    );
    this.root.appendChild(head);
    this.root.appendChild(this.alarm);
    this.section('lists', [], this.sources);
    this.section('allowed.title', ['allowed.hint'], this.allowed);
    this.section('blocked.title', ['blocked.hint'], this.blocked);
    this.root.appendChild(this.apply);
    this.root.appendChild(this.labelled(element('p', 'foot'), 'youtube'));
    this.labelled(this.update, 'update');
    this.labelled(this.apply, 'apply');
    this.sign.dir = 'ltr';
    this.sign.setAttribute('aria-hidden', 'true');
    this.alarm.setAttribute('role', 'alert');
    this.note.setAttribute('role', 'status');
    [this.allowed, this.blocked].forEach((field) => {
      field.rows = 3;
      field.spellcheck = false;
      field.dir = 'ltr';
    });
  }

  private section(title: Key, hints: ReadonlyArray<Key>, body: HTMLElement): void {
    const part = element('section', '');
    const heading = this.labelled(element('h2', ''), title);
    heading.id = `title-${title}`;
    part.appendChild(heading);
    hints.forEach((hint) => part.appendChild(this.labelled(element('p', 'hint'), hint)));
    body.setAttribute('aria-labelledby', heading.id);
    part.appendChild(body);
    this.root.appendChild(part);
  }

  /** The element that always shows the text in the current language. */
  private labelled<T extends HTMLElement>(node: T, key: Key): T {
    this.labels.push({ node, key });
    return node;
  }

  /** Does the work while the buttons are disabled, and shows what has come out of it. */
  private run(doing: Key, work: () => Promise<void>): void {
    this.alarm.classList.add('hidden');
    this.note.textContent = this.report.words[doing];
    this.lock(true);
    work()
      .catch((error: unknown) => this.complain(String(error)))
      .then(() => this.backend.state())
      .then((report) => this.renewed(report))
      .catch((error: unknown) => {
        this.note.textContent = '';
        this.complain(String(error));
      })
      .then(() => this.lock(false))
      .catch((error: unknown) => this.complain(String(error)));
  }

  private lock(busy: boolean): void {
    [this.power, this.update, this.apply].forEach((button) => {
      button.disabled = busy;
    });
    this.speech.disabled = busy;
    this.root.classList.toggle('busy', busy);
  }

  private complain(text: string): void {
    this.alarm.textContent = text;
    this.alarm.classList.remove('hidden');
  }

  /** Shows the new report. What a person has typed and not applied yet stays in the form. */
  private renewed(report: Report): void {
    const typed = this.form();
    const kept = alike(typed, saved(this.report)) ? saved(report) : typed;
    this.report = report;
    this.fill(kept);
    this.show(report);
  }

  private fill(settings: Settings): void {
    this.sources.textContent = '';
    this.report.rows.forEach((row) =>
      this.sources.appendChild(this.source(row, settings.sources.indexOf(row.id) >= 0)),
    );
    this.allowed.value = settings.allowed.join('\n');
    this.blocked.value = settings.blocked.join('\n');
  }

  private show(report: Report): void {
    const on = report.count > 0;
    document.documentElement.lang = report.spoken;
    document.documentElement.dir = report.words.direction;
    this.labels.forEach((label) => {
      label.node.textContent = report.words[label.key];
    });
    this.sign.textContent = formula(report);
    this.title.textContent = report.words[on ? 'title.on' : 'title.off'];
    this.note.textContent = summary(report);
    this.power.textContent = report.words[on ? 'turn.off' : 'turn.on'];
    this.root.classList.toggle('on', on);
    this.speech.title = report.words.speech;
    this.speech.setAttribute('aria-label', report.words.speech);
    this.speech.textContent = '';
    [{ code: '', title: report.words['speech.system'] }].concat(report.languages).forEach((tongue) => {
      const option = written('option', '', tongue.title);
      option.value = tongue.code;
      this.speech.appendChild(option);
    });
    this.speech.value = report.settings.language;
    this.mark();
  }

  private source(row: Row, on: boolean): HTMLElement {
    const label = element('label', 'source');
    const box = element('input', '');
    const text = element('span', 'text');
    box.type = 'checkbox';
    box.checked = on;
    box.value = row.id;
    text.appendChild(written('b', '', row.title));
    text.appendChild(written('span', 'hint', `${row.note}; ${age(row, this.report)}`));
    label.appendChild(box);
    label.appendChild(text);
    return label;
  }

  /** What a person has typed into the form so far. */
  private form(): Settings {
    const boxes: ReadonlyArray<HTMLInputElement> = Array.from(
      this.sources.getElementsByTagName('input'),
    );
    return {
      sources: boxes.filter((box) => box.checked).map((box) => box.value),
      blocked: new Lines(this.blocked.value).list(),
      allowed: new Lines(this.allowed.value).list(),
      language: this.report.settings.language,
    };
  }

  /** Shows the button that applies the choice if the choice differs from the saved one. */
  private mark(): void {
    this.apply.classList.toggle('hidden', alike(this.form(), saved(this.report)));
  }

  private store(): Promise<void> {
    return this.backend.save(this.form());
  }

  /** Saves the choice, downloads the missing lists and blocks the addresses. */
  private enable(): Promise<void> {
    const chosen = this.form().sources;
    const missing = this.report.rows.some(
      (row) => row.updated === 0 && chosen.indexOf(row.id) >= 0,
    );
    return this.store()
      .then(() => (missing ? this.backend.refresh() : []))
      .then((failures) => this.backend.turn(true).then(() => this.mention(failures)));
  }

  /** Downloads the lists again. If blocking is on, applies them at once. */
  private renew(): Promise<void> {
    return this.store()
      .then(() => this.backend.refresh())
      .then((failures) =>
        (this.report.count > 0 ? this.backend.turn(true) : Promise.resolve()).then(() =>
          this.mention(failures),
        ),
      );
  }

  private mention(failures: ReadonlyArray<string>): void {
    if (failures.length > 0) {
      this.complain(failures.join(' '));
    }
  }
}
