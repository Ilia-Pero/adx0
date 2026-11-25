import { asked } from './browser.ts';

// If the extension has just been updated, there is nobody to answer, and the page stays as it is.
asked({ kind: 'page' }).catch(() => 'nobody has answered');
