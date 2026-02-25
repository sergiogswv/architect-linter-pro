import { Service8 } from '../services/service_8';

export class Controller8 {
  private service = new Service8();

  handle() {
    return this.service.doWork();
  }
}
