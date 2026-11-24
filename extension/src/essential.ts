import essential from '../../data/essential.json' with { type: 'json' };

/**
 * Sites that lists cannot block: one of them in a list is a mistake of the list. Ad subdomains
 * of these sites are blocked as usual.
 */
const ESSENTIAL: ReadonlySet<string> = new Set(essential);

/** Addresses without essential sites. */
export function harmless(names: Iterable<string>): ReadonlyArray<string> {
  return Array.from(names).filter((name) => !ESSENTIAL.has(name.replace(/^www\./, '')));
}
