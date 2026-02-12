import { ProductRepository } from './product.repository'; // ❌ VIOLATION

/**
 * Product Controller
 * Another example of controller violating layer architecture
 */
export class ProductController {
  private productRepo: ProductRepository; // ❌ Should use service

  constructor() {
    this.productRepo = new ProductRepository();
  }

  getProducts() {
    return this.productRepo.findAll(); // ❌ Direct repository access
  }

  getProduct(id: number) {
    return this.productRepo.findById(id); // ❌ Direct repository access
  }
}
