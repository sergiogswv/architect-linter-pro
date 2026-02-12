import { C } from './c';

/**
 * E imports C - completing the cycle C -> D -> E -> C
 */
export class E {
  private c: C;

  constructor() {
    this.c = new C();
  }

  fromE() {
    return this.c.fromC();
  }
}
