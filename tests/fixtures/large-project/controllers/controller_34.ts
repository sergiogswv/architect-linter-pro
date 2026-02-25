import { Service34 } from '../services/service_34';

export class Controller34 {
  private service = new Service34();

  handle() {
    return this.service.doWork();
  }
}
