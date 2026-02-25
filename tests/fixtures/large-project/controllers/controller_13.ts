import { Service13 } from '../services/service_13';

export class Controller13 {
  private service = new Service13();

  handle() {
    return this.service.doWork();
  }
}
