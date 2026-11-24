import type { Message } from './browser.ts';

const opened: Message = { kind: 'page' };

// If the extension has just been updated, there is nobody to answer, and the page stays as it is.
browser.runtime.sendMessage<void>(opened).catch(() => 'nobody has answered');
