import { Service15 } from '../services/service_15';

export class Controller15 {
  private service = new Service15();

  handle() {
    return this.service.doWork();
  }
}
