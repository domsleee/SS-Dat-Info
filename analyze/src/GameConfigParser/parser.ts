import { GameObject, GameValue, Vector, ParserPosition } from "./types";
import { ParseError } from "./errors";
import { InputReader } from "./input-reader";

export class GameConfigParser {
  private reader: InputReader;

  constructor(input: string | InputReader) {
    this.reader = input instanceof InputReader ? input : new InputReader(input);
  }

  private parseIdentifier(): string {
    let result = "";
    while (this.reader.current() && /[a-zA-Z0-9_]/.test(this.reader.current()!)) {
      result += this.reader.current();
      this.reader.advance();
    }
    if (result.length === 0) {
      throw new ParseError("Expected identifier", this.reader.line);
    }
    return result;
  }

  private parseString(): string {
    this.reader.advance(); // Skip opening quote
    let result = "";

    while (this.reader.current() && this.reader.current() !== '"') {
      result += this.reader.current();
      this.reader.advance();
    }

    if (this.reader.current() !== '"') {
      throw new ParseError("Unterminated string", this.reader.line);
    }

    this.reader.advance(); // Skip closing quote
    return result;
  }

  private parseVectorContents(): Vector[] {
    const values: Vector[] = [];
    let currentValue = "";

    const pushCurrentValue = () => {
      if (currentValue.trim()) {
        values.push(currentValue.trim());
        currentValue = "";
      }
    };

    while (this.reader.current() && this.reader.current() !== "}") {
      if (this.reader.current() === "{") {
        this.reader.advance();
        values.push(this.parseVectorContents());
      } else if (this.reader.current() === ",") {
        pushCurrentValue();
        this.reader.advance();
      } else if (/[\s\t\n\r]/.test(this.reader.current()!)) {
        pushCurrentValue();
        this.reader.advance();
      } else {
        currentValue += this.reader.current();
        this.reader.advance();
      }
    }

    pushCurrentValue();

    if (this.reader.current() === "}") {
      this.reader.advance();
    } else {
      throw new ParseError("Expected } in vector", this.reader.line);
    }

    return values;
  }

  private parseVector(): Vector {
    this.reader.advance(); // Skip opening brace
    return this.parseVectorContents();
  }

  private parseValue(): GameValue {
    this.reader.skipWhitespace();

    if (!this.reader.current()) {
      throw new ParseError("Unexpected end of input while parsing value", this.reader.line);
    }

    if (this.reader.current() === "{") {
      return this.parseVector();
    } else if (this.reader.current() === '"') {
      return this.parseString();
    } else {
      // Parse until semicolon or whitespace
      let result = "";
      while (this.reader.current() && !/[\s\t\n\r;]/.test(this.reader.current()!)) {
        result += this.reader.current();
        this.reader.advance();
      }
      return result;
    }
  }

  private parseProperty(): { name: string; value: GameValue } {
    this.reader.skipWhitespace();

    const name = this.parseIdentifier();

    this.reader.skipWhitespace();
    if (this.reader.current() !== "=") {
      throw new ParseError(`Expected = after property name "${name}"`, this.reader.line);
    }
    this.reader.advance();

    const value = this.parseValue();

    // Handle optional semicolon
    this.reader.skipWhitespace();
    if (this.reader.current() === ";") this.reader.advance();

    return { name, value };
  }

  private parseObject(): GameObject {
    this.reader.skipWhitespace();

    const name = this.parseIdentifier();

    this.reader.skipWhitespace();
    if (this.reader.current() !== "{") {
      throw new ParseError(`Expected { after object name "${name}"`, this.reader.line);
    }
    this.reader.advance();

    const properties: Record<string, GameValue> = {};
    while (this.reader.current() && this.reader.current() !== "}") {
      const { name: propName, value } = this.parseProperty();
      properties[propName] = value;
      this.reader.skipWhitespace();
    }

    if (this.reader.current() !== "}") {
      throw new ParseError(`Expected } to close object "${name}"`, this.reader.line);
    }
    this.reader.advance();

    // Handle optional semicolon after closing brace
    this.reader.skipWhitespace();
    if (this.reader.current() === ";") this.reader.advance();

    return {
      type: name,
      properties,
    };
  }

  public parse(): GameObject[] {
    const objects: GameObject[] = [];

    while (this.reader.pos < this.reader.input.length) {
      this.reader.skipWhitespace();
      if (!this.reader.current()) break;

      objects.push(this.parseObject());
    }

    return objects;
  }

  public getPosition(): ParserPosition {
    return this.reader.getPosition();
  }
}
