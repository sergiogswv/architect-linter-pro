import { Service21 } from '../services/service_21';

export class Controller21 {
  private service = new Service21();

  handle() {
    return this.service.doWork();
  }
}
