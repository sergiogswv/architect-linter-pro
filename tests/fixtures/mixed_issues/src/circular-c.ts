import { CircularA } from './circular-a';

/**
 * Part of circular dependency: A -> B -> C -> A
 * This completes the cycle!
 */
export class CircularC {
  private a: CircularA;

  constructor() {
    this.a = new CircularA();
  }

  methodC() {
    return this.a.methodA();
  }
}
