import { Service10 } from '../services/service_10';

export class Controller10 {
  private service = new Service10();

  handle() {
    return this.service.doWork();
  }
}
