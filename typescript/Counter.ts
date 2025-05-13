/**
 * counter of the comparison operations which pretends that one comparion takes 1 seconds.
 */
class Counter {
  #count: number;
  #limit: number;

  constructor(limit: number = 3600) {
    this.#count = 0;
    this.#limit = limit;
  }

  increment(): void {
    this.#count++;
    if (this.#count >= this.#limit) {
      throw new Error(`Abort at count ${this.#count}. that corresponds to ${this}!`);
    }
  }

  get count(): number {
    return this.#count;
  }

  toString(): string {
    let count = this.#count;
    if (count < 60) {
      return count != 1 ? `${count} seconds` : `${count} second`;
    }
    count = Math.floor((count + 30) / 60);
    if (count < 60) {
      return count != 1 ? `${count} minutes` : `${count} minute`;
    }
    count = Math.floor((count + 30) / 60);
    if (count < 24) {
      return count != 1 ? `${count} hours` : `${count} hour`;
    }
    count = Math.floor((count + 12) / 24);
    if (count < 366) {
      return count != 1 ? `${count} days` : `${count} day`;
    }
    count = Math.floor((count + 183) / 366);
    return count != 1 ? `${count} years` : `${count} year`;
  }

}


export { Counter };
