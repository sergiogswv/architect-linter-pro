import { B } from './b';

/**
 * Class A - imports B, which imports A
 * This creates a circular dependency!
 */
export class A {
  private b: B;

  constructor() {
    this.b = new B();
  }

  doSomething() {
    return this.b.doSomethingElse();
  }

  methodA() {
    return 'A';
  }
}
