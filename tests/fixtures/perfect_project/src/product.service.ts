/**
 * Product Service - clean service layer
 */
export class ProductService {
  private products: any[] = [];

  findById(id: number) {
    return this.products.find(p => p.id === id);
  }

  findAll() {
    return this.products;
  }

  create(data: { name: string; price: number }) {
    const product = {
      id: this.products.length + 1,
      ...data
    };
    this.products.push(product);
    return product;
  }
}
