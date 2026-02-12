import { E } from './e';

/**
 * D imports E
 */
export class D {
  private e: E;

  constructor() {
    this.e = new E();
  }

  fromD() {
    return this.e.fromE();
  }
}
