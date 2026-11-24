// The spare way for YouTube: if a video ad has started after all, rewinds it to the end and presses
// "Skip". It watches the class of the player only, and there are no timers.

const SKIP = '.ytp-skip-ad-button, .ytp-ad-skip-button, .ytp-ad-skip-button-modern';

const SLOTS = [
  'ytd-ad-slot-renderer',
  'ytd-in-feed-ad-layout-renderer',
  'ytd-promoted-sparkles-web-renderer',
  'ytd-display-ad-renderer',
  'ytd-banner-promo-renderer',
  '#masthead-ad',
  '.ytp-ad-overlay-container',
];

function skip(player: Element): void {
  if (!player.classList.contains('ad-showing')) {
    return;
  }
  const video = player.querySelector('video');
  if (video !== null && Number.isFinite(video.duration) && video.duration > 0) {
    video.currentTime = video.duration;
  }
  const button = player.querySelector<HTMLElement>(SKIP);
  if (button !== null) {
    button.click();
  }
}

function watch(player: Element): void {
  new MutationObserver(() => skip(player)).observe(player, {
    attributes: true,
    attributeFilter: ['class'],
  });
  player.addEventListener('loadedmetadata', () => skip(player), true);
  skip(player);
}

/** Finds the player. On pages without it, waits until it appears. */
function find(): void {
  const player = document.querySelector('#movie_player');
  if (player !== null) {
    watch(player);
    return;
  }
  const waiting = new MutationObserver(() => {
    const late = document.querySelector('#movie_player');
    if (late !== null) {
      waiting.disconnect();
      watch(late);
    }
  });
  waiting.observe(document.documentElement, { childList: true, subtree: true });
}

const style = document.createElement('style');
style.textContent = SLOTS.map((slot) => `${slot}{display:none!important}`).join('\n');
document.documentElement.appendChild(style);
find();
