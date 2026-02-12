import { CircularC } from './circular-c';

/**
 * Part of circular dependency: A -> B -> C -> A
 */
export class CircularB {
  private c: CircularC;

  constructor() {
    this.c = new CircularC();
  }

  methodB() {
    return this.c.methodC();
  }
}
