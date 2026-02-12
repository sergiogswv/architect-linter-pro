/**
 * Product Repository
 */
export class ProductRepository {
  private products: any[] = [];

  findById(id: number) {
    return this.products.find(p => p.id === id);
  }

  findAll() {
    return this.products;
  }

  create(data: any) {
    const product = {
      id: this.products.length + 1,
      ...data
    };
    this.products.push(product);
    return product;
  }
}
