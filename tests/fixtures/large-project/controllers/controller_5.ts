import { Service5 } from '../services/service_5';

export class Controller5 {
  private service = new Service5();

  handle() {
    return this.service.doWork();
  }
}
