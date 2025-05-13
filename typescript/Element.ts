import { Counter } from './Counter.ts';


/**
 * a wrapper around integers which counts the number of comparison and value accesses.
 * - it also ensures that integers are in a range from 0 to 65535 (uint16).
 */
class ListElement {
  #counter: Counter;
  #value: number;

  constructor(counter: Counter, value: number) {
    if (!Number.isInteger(value)) {
      throw new Error(`Element value must be an integer, not ${typeof value}`);
    }
    if (value < 0 || value >= 2**16) {
      throw new Error(`Element value must be uint 16 (0-65535), not ${value}`);
    }
    this.#counter = counter;
    this.#value = value;
  }

  /**
   * compare two ListElement objects.
   * @param a first element.
   * @param b second element.
   * @returns -1 if a < b, 0 if a == b, 1 if a > b.
   */
  static compare(a: ListElement, b: ListElement): number {
    return a.compareTo(b);
  }

  compareTo(other: ListElement): number {
    return Math.sign(this.#value - other.toInteger());
  }

  lessThan(other: ListElement): boolean {
    return this.compareTo(other) < 0;
  }

  lessEqual(other: ListElement): boolean {
    return this.compareTo(other) <= 0;
  }

  equal(other: ListElement): boolean {
    return this.compareTo(other) == 0;
  }

  greaterEqual(other: ListElement): boolean {
    return this.compareTo(other) >= 0;
  }

  greaterThan(other: ListElement): boolean {
    return this.compareTo(other) > 0;
  }

  toInteger(): number {
    this.#counter.increment();
    return this.#value;
  }

  toString(): string {
    return this.#value.toString();
  }

}


export { ListElement };
