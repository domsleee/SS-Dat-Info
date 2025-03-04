import { ParserPosition } from "./types";

export class InputReader {
  private input: string;
  pos: number;
  line: number;

  constructor(input: string) {
    this.input = input;
    this.pos = 0;
    this.line = 1;
  }

  current(): string | null {
    return this.pos < this.input.length ? this.input[this.pos] : null;
  }

  advance(): InputReader {
    if (this.current() === "\n") this.line++;
    this.pos++;
    return this;
  }

  skipWhitespace(): InputReader {
    while (this.current() && /[\s\t\n\r]/.test(this.current()!)) {
      this.advance();
    }
    return this;
  }

  getPosition(): ParserPosition {
    return {
      pos: this.pos,
      line: this.line,
      current: this.current(),
    };
  }

  static async fromFile(filePath: string): Promise<InputReader> {
    // In Node.js:
    // const fs = require('fs');
    // const content = fs.readFileSync(filePath, 'utf8');
    // return new InputReader(content);

    // For browser:
    const response = await fetch(filePath);
    const content = await response.text();
    return new InputReader(content);
  }
}
