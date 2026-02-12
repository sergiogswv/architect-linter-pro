import { A } from './a';

/**
 * Class B - imports A, which imports B
 * This completes the circular dependency!
 */
export class B {
  private a: A;

  constructor() {
    this.a = new A();
  }

  doSomethingElse() {
    return this.a.methodA();
  }

  methodB() {
    return 'B';
  }
}
