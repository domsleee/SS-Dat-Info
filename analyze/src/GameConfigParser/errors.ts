export class ParseError extends Error {
  line: number;
    
  constructor(message: string, line: number) {
    super(`Line ${line}: ${message}`);
    this.line = line;
  }
}
