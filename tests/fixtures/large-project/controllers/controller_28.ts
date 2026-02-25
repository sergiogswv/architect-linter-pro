import { Service28 } from '../services/service_28';

export class Controller28 {
  private service = new Service28();

  handle() {
    return this.service.doWork();
  }
}
