import { expect } from "bun:test";

expect.extend({
  toBeCloseToStructure(received: unknown, expected: unknown, precision = 3) {
    const compare = (a: unknown, b: unknown): boolean => {
      if (Array.isArray(a) && Array.isArray(b)) {
        return a.length === b.length && a.every((item, i) => compare(item, b[i]));
      } else if (typeof a === 'object' && a !== null && typeof b === 'object' && b !== null) {
        return Object.keys(b).every(key => 
          Object.prototype.hasOwnProperty.call(a, key) && compare((a as Record<string, unknown>)[key], (b as Record<string, unknown>)[key])
        );
      } else if (typeof a === 'number' && typeof b === 'number') {
        return Math.abs(a - b) < Math.pow(10, -precision);
      } else {
        return this.equals(a, b);
      }
    };
    
    const pass = compare(received, expected);
    
    return {
      pass,
      message: () => 
        pass
          ? `Expected ${this.utils.printReceived(received)} not to be close to ${this.utils.printExpected(expected)}`
          : `Expected ${this.utils.printReceived(received)} to be close to ${this.utils.printExpected(expected)}`
    };
  }
});

declare module "bun:test" {
  interface Matchers {
    toBeCloseToStructure(expected: unknown, precision?: number): void;
  }
}