import { Service1 } from '../services/service_1';

export class Controller1 {
  private service = new Service1();

  handle() {
    return this.service.doWork();
  }
}
