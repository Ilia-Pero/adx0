import { Sandbox } from './sandbox.ts';
import { start } from './start.ts';

// The language may be given in the address: http://127.0.0.1:5392/?he
start(new Sandbox(window.location.search.slice(1) || navigator.language), document.body);
