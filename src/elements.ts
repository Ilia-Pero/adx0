/** An element of the page. */
export function element<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  name: string,
): HTMLElementTagNameMap[K] {
  const node = document.createElement(tag);
  node.className = name;
  return node;
}

/** An element of the page with a text in it. */
export function written<K extends keyof HTMLElementTagNameMap>(
  tag: K,
  name: string,
  text: string,
): HTMLElementTagNameMap[K] {
  const node = element(tag, name);
  node.textContent = text;
  return node;
}
