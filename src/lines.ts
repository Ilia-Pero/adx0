/** Lines typed into a field: without empty ones, spaces around and repeats. */
export class Lines {
  private readonly text: string;

  constructor(text: string) {
    this.text = text;
  }

  list(): ReadonlyArray<string> {
    return this.text
      .split(/[\s,;]+/)
      .filter((line, index, all) => line !== '' && all.indexOf(line) === index);
  }
}
