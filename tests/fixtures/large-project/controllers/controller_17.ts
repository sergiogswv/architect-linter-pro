import { Service17 } from '../services/service_17';

export class Controller17 {
  private service = new Service17();

  handle() {
    return this.service.doWork();
  }
}
