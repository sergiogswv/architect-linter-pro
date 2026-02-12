import { ProductService } from './product.service';

/**
 * Product Controller - another clean controller
 */
export class ProductController {
  private productService: ProductService;

  constructor() {
    this.productService = new ProductService();
  }

  getProduct(id: number) {
    return this.productService.findById(id);
  }

  listProducts() {
    return this.productService.findAll();
  }

  addProduct(name: string, price: number) {
    return this.productService.create({ name, price });
  }
}
