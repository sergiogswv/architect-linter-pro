import { CircularB } from './circular-b';

/**
 * Part of circular dependency: A -> B -> C -> A
 */
export class CircularA {
  private b: CircularB;

  constructor() {
    this.b = new CircularB();
  }

  methodA() {
    return this.b.methodB();
  }
}
