import { pruned } from './pruned.ts';

// Works inside the page of YouTube: removes the schedule of ads from the answers the player gets.
// An answer comes in three ways: in the page itself, through JSON.parse and through fetch.

const parse = JSON.parse.bind(JSON);
JSON.parse = (text: string, reviver?: (key: string, value: unknown) => unknown): unknown =>
  pruned<unknown>(parse(text, reviver));

const json = Response.prototype.json;
Response.prototype.json = function patched(this: Response): Promise<unknown> {
  return json.call(this).then((answer: unknown) => pruned(answer));
};

let first: unknown;
Object.defineProperty(window, 'ytInitialPlayerResponse', {
  configurable: true,
  get: () => first,
  set: (answer: unknown) => {
    first = pruned(answer);
  },
});
