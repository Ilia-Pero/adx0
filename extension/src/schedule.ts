/** How often the lists are downloaded, in minutes: once a day. */
export const PERIOD = 24 * 60;

const MINUTE = 60_000;

/**
 * In how many minutes to download the lists that were downloaded at the moment, which is zero if
 * they were not: then they are being downloaded now. Overdue lists wait for a minute, because
 * a browser that has just been started has enough to do.
 */
export function due(updated: number, now: number): number {
  if (updated === 0) {
    return PERIOD;
  }
  return Math.min(Math.max(PERIOD - (now - updated) / MINUTE, 1), PERIOD);
}
