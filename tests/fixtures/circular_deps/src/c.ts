import { D } from './d';

/**
 * Another circular dependency: C -> D -> E -> C
 */
export class C {
  private d: D;

  constructor() {
    this.d = new D();
  }

  fromC() {
    return this.d.fromD();
  }
}
