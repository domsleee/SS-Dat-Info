import { expect } from "bun:test";

expect.extend({
  toBeCloseToStructure(received: any, expected: any, precision = 3) {
    const compare = (a: any, b: any): boolean => {
      if (Array.isArray(a) && Array.isArray(b)) {
        return a.length === b.length && a.every((item, i) => compare(item, b[i]));
      } else if (typeof a === 'object' && a !== null && typeof b === 'object' && b !== null) {
        return Object.keys(b).every(key => 
          Object.prototype.hasOwnProperty.call(a, key) && compare(a[key], b[key])
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
  interface Matchers<T> {
    toBeCloseToStructure(expected: any, precision?: number): void;
  }
}