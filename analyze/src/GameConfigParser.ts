export type Vector = Vector[] | string;
export type GameValue = Vector | string;

export interface GameObject {
    type: string;
    properties: Record<string, GameValue>;
}

export interface ParserPosition {
    pos: number;
    line: number;
    current: string | null;
}

export class ParseError extends Error {
    line: number;
    
    constructor(message: string, line: number) {
        super(`Line ${line}: ${message}`);
        this.line = line;
    }
}

export class GameConfigParser {
    private pos: number;
    private line: number;
    private current: string | null;
    private readonly input: string;

    constructor(input: string) {
        this.input = input;
        this.pos = 0;
        this.line = 1;
        this.current = input[0] || null;
    }

    private advance(): void {
        this.pos++;
        if (this.current === '\n') this.line++;
        this.current = this.pos < this.input.length ? this.input[this.pos] : null;
    }

    private skipWhitespace(): void {
        while (this.current && /[\s\t\n\r]/.test(this.current)) {
            this.advance();
        }
    }

    private parseIdentifier(): string {
        let result = '';
        while (this.current && /[a-zA-Z0-9_]/.test(this.current)) {
            result += this.current;
            this.advance();
        }
        if (result.length === 0) {
            throw new ParseError('Expected identifier', this.line);
        }
        return result;
    }

    private parseString(): string {
        this.advance(); // Skip opening quote
        let result = '';
        
        while (this.current && this.current !== '"') {
            result += this.current;
            this.advance();
        }
        
        if (this.current !== '"') {
            throw new ParseError('Unterminated string', this.line);
        }
        
        this.advance(); // Skip closing quote
        return result;
    }

    private parseVectorContents(): Vector[] {
        const values: Vector[] = [];
        let currentValue = '';
        
        const pushCurrentValue = () => {
            if (currentValue.trim()) {
                values.push(currentValue.trim());
                currentValue = '';
            }
        };
        
        while (this.current && this.current !== '}') {
            if (this.current === '{') {
                this.advance();
                values.push(this.parseVectorContents());
            } else if (this.current === ',') {
                pushCurrentValue();
                this.advance();
            } else if (/[\s\t\n\r]/.test(this.current)) {
                pushCurrentValue();
                this.advance();
            } else {
                currentValue += this.current;
                this.advance();
            }
        }
        
        pushCurrentValue();
        
        if (this.current === '}') {
            this.advance();
        } else {
            throw new ParseError('Expected } in vector', this.line);
        }
        
        return values;
    }

    private parseVector(): Vector {
        this.advance(); // Skip opening brace
        return this.parseVectorContents();
    }

    private parseValue(): GameValue {
        this.skipWhitespace();
        
        if (!this.current) {
            throw new ParseError('Unexpected end of input while parsing value', this.line);
        }
        
        if (this.current === '{') {
            return this.parseVector();
        } else if (this.current === '"') {
            return this.parseString();
        } else {
            // Parse until semicolon or whitespace
            let result = '';
            while (this.current && !/[\s\t\n\r;]/.test(this.current)) {
                result += this.current;
                this.advance();
            }
            return result;
        }
    }

    private parseProperty(): { name: string, value: GameValue } {
        this.skipWhitespace();
        
        const name = this.parseIdentifier();
        
        this.skipWhitespace();
        if (this.current !== '=') {
            throw new ParseError(`Expected = after property name "${name}"`, this.line);
        }
        this.advance();
        
        const value = this.parseValue();
        
        // Handle optional semicolon
        this.skipWhitespace();
        if (this.current === ';') this.advance();
        
        return { name, value };
    }

    private parseObject(): GameObject {
        this.skipWhitespace();
        
        const name = this.parseIdentifier();
        
        this.skipWhitespace();
        if (this.current !== '{') {
            throw new ParseError(`Expected { after object name "${name}"`, this.line);
        }
        this.advance();

        const properties: Record<string, GameValue> = {};
        while (this.current && this.current !== '}') {
            const { name: propName, value } = this.parseProperty();
            properties[propName] = value;
            this.skipWhitespace();
        }

        if (this.current !== '}') {
            throw new ParseError(`Expected } to close object "${name}"`, this.line);
        }
        this.advance();
        
        // Handle optional semicolon after closing brace
        this.skipWhitespace();
        if (this.current === ';') this.advance();

        return {
            type: name,
            properties
        };
    }

    public parse(): GameObject[] {
        const objects: GameObject[] = [];
        
        while (this.pos < this.input.length) {
            this.skipWhitespace();
            if (!this.current) break;
            
            objects.push(this.parseObject());
        }
        
        return objects;
    }

    public getPosition(): ParserPosition {
        return {
            pos: this.pos,
            line: this.line,
            current: this.current
        };
    }
}