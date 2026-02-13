import { ModuleA } from './module_a';

export class ModuleB {
  private a: ModuleA | null = null;

  getValue(): string {
    return 'B';
  }

  setA(a: ModuleA): void {
    this.a = a;
  }
}
