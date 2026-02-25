import { Service11 } from '../services/service_11';

export class Controller11 {
  private service = new Service11();

  handle() {
    return this.service.doWork();
  }
}
