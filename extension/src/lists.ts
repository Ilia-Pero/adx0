/** How a list is written. */
export type Format =
  /** Lines such as "0.0.0.0 ads.example.com" or just "ads.example.com". */
  | 'hosts'
  /** Rules of Adblock Plus. */
  | 'adblock';

/** A list of filters on the internet. */
export interface List {
  readonly title: string;
  readonly url: string;
  readonly format: Format;
  /** Whether to take the rules that hide elements on pages. */
  readonly cosmetic: boolean;
  /** Codes of the languages the list is for. Empty means that it is for everyone. */
  readonly languages: ReadonlyArray<string>;
}

const REGIONAL = 'https://easylist-downloads.adblockplus.org';

/**
 * Lists the extension downloads. Only addresses are taken from EasyList: thousands of its
 * selectors for every site would slow down every page.
 */
const LISTS: ReadonlyArray<List> = [
  {
    title: 'Peter Lowe',
    url: 'https://pgl.yoyo.org/adservers/serverlist.php?hostformat=hosts&showintro=0&mimetype=plaintext',
    format: 'hosts',
    cosmetic: false,
    languages: [],
  },
  {
    title: 'AdAway',
    url: 'https://adaway.org/hosts.txt',
    format: 'hosts',
    cosmetic: false,
    languages: [],
  },
  {
    title: 'EasyList',
    url: 'https://easylist.to/easylist/easylist.txt',
    format: 'adblock',
    cosmetic: false,
    languages: [],
  },
  regional('EasyList Germany', `${REGIONAL}/easylistgermany.txt`, ['de']),
  regional('Greek AdBlock Filter', 'https://www.void.gr/kargig/void-gr-filters.txt', ['el']),
  regional('EasyList Spanish', `${REGIONAL}/easylistspanish.txt`, ['es']),
  regional('Liste FR', `${REGIONAL}/liste_fr.txt`, ['fr']),
  regional('EasyList Hebrew', `${REGIONAL}/israellist.txt`, ['he', 'iw']),
  regional('EasyList Italy', `${REGIONAL}/easylistitaly.txt`, ['it']),
  regional(
    'AdGuard Japanese',
    'https://filters.adtidy.org/extension/chromium/filters/7.txt',
    ['ja'],
  ),
  regional('KoreanList', `${REGIONAL}/koreanlist.txt`, ['ko']),
  regional('RU AdList', `${REGIONAL}/advblock.txt`, ['ru', 'uk', 'be', 'kk', 'uz']),
  regional('EasyList China', `${REGIONAL}/easylistchina.txt`, ['zh']),
];

function regional(title: string, url: string, languages: ReadonlyArray<string>): List {
  return { title, url, format: 'adblock', cosmetic: true, languages };
}

/** Lists for a person who reads the languages with the tags, such as en-US. */
export function suitable(tags: ReadonlyArray<string>): ReadonlyArray<List> {
  const codes = new Set(tags.map((tag) => tag.toLowerCase().replace(/[-_].*$/, '')));
  return LISTS.filter(
    (list) =>
      list.languages.length === 0 || list.languages.some((language) => codes.has(language)),
  );
}
