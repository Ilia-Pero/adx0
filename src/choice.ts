import type { Report, Settings } from './backend.ts';

/** What the window offers to choose, the way it was last saved. */
export function saved(report: Report): Settings {
  return {
    sources: report.rows.filter((row) => row.on).map((row) => row.id),
    blocked: report.settings.blocked,
    allowed: report.settings.allowed,
    language: report.settings.language,
  };
}

/** The same choice in another language. */
export function translated(settings: Settings, language: string): Settings {
  return {
    sources: settings.sources,
    blocked: settings.blocked,
    allowed: settings.allowed,
    language,
  };
}

function equal(one: ReadonlyArray<string>, other: ReadonlyArray<string>): boolean {
  return one.length === other.length && one.every((item, index) => item === other[index]);
}

/** True if both choices block the same. */
export function alike(one: Settings, other: Settings): boolean {
  return (
    equal(one.sources, other.sources) &&
    equal(one.blocked, other.blocked) &&
    equal(one.allowed, other.allowed)
  );
}
