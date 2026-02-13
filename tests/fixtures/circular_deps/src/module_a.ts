import { ModuleB } from './module_b';

export class ModuleA {
  private b: ModuleB;

  constructor() {
    this.b = new ModuleB();
  }

  doSomething(): string {
    return 'A uses ' + this.b.getValue();
  }
}
