/** Fields in which YouTube sends the schedule of ads to the player. */
const ADS: ReadonlyArray<string> = ['adPlacements', 'adSlots', 'playerAds', 'adBreakHeartbeatParams'];

function record(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

/**
 * The answer of YouTube without the schedule of ads. Other answers are returned as they are.
 * The object changes in place, because the player holds a reference to it.
 */
export function pruned<T>(answer: T): T {
  if (record(answer)) {
    const fields: Record<string, unknown> = answer;
    ADS.forEach((name) => {
      if (name in fields) {
        delete fields[name];
      }
    });
    if ('playerResponse' in fields) {
      pruned(fields['playerResponse']);
    }
  }
  return answer;
}
