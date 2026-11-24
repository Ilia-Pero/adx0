/** A rule of the browser: what to do with a request. */
export interface Rule {
  readonly id: number;
  readonly priority: number;
  readonly action: { readonly type: 'block' | 'allowAllRequests' };
  readonly condition: {
    readonly requestDomains: ReadonlyArray<string>;
    readonly urlFilter?: string;
    readonly domainType?: 'thirdParty';
    readonly resourceTypes?: ReadonlyArray<string>;
    readonly excludedResourceTypes?: ReadonlyArray<string>;
  };
}

/** Numbers the rules may have in the browser. */
export const NUMBERS: ReadonlyArray<number> = [1, 2, 3, 4];

/**
 * Rules of the browser. Thousands of sites fit into one rule: the browser finds a site in it
 * by hash, so the speed does not depend on their number.
 */
export class Rules {
  private readonly everywhere: ReadonlyArray<string>;
  private readonly foreign: ReadonlyArray<string>;
  private readonly allowed: ReadonlyArray<string>;

  constructor(
    everywhere: ReadonlyArray<string>,
    foreign: ReadonlyArray<string>,
    allowed: ReadonlyArray<string>,
  ) {
    this.everywhere = everywhere;
    this.foreign = foreign;
    this.allowed = allowed;
  }

  list(): ReadonlyArray<Rule> {
    const rules: ReadonlyArray<Rule> = [
      {
        id: 1,
        priority: 1,
        action: { type: 'block' },
        condition: {
          requestDomains: this.everywhere,
          excludedResourceTypes: ['main_frame'],
        },
      },
      {
        id: 2,
        priority: 1,
        action: { type: 'block' },
        condition: {
          requestDomains: this.foreign,
          domainType: 'thirdParty',
          excludedResourceTypes: ['main_frame'],
        },
      },
      {
        id: 3,
        priority: 10,
        action: { type: 'allowAllRequests' },
        condition: {
          requestDomains: this.allowed,
          resourceTypes: ['main_frame'],
        },
      },
      {
        // The ad script of Yandex is on the same site as its search, so a folder is blocked.
        id: 4,
        priority: 1,
        action: { type: 'block' },
        condition: {
          requestDomains: this.everywhere.length === 0 ? [] : ['yandex.ru', 'ya.ru', 'yandex.com'],
          urlFilter: '/ads/system/',
          excludedResourceTypes: ['main_frame'],
        },
      },
    ];
    return rules.filter((rule) => rule.condition.requestDomains.length > 0);
  }
}
