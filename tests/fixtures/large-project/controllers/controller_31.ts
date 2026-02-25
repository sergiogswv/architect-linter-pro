import { Service31 } from '../services/service_31';

export class Controller31 {
  private service = new Service31();

  handle() {
    return this.service.doWork();
  }
}
