/**
 * Bad Repository
 */
export class BadRepository {
  private data: any[] = [];

  findAll() {
    return this.data;
  }

  findById(id: number) {
    return this.data.find(d => d.id === id);
  }

  create(item: any) {
    this.data.push(item);
    return item;
  }
}
