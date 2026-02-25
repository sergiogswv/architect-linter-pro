import { Service9 } from '../services/service_9';

export class Controller9 {
  private service = new Service9();

  handle() {
    return this.service.doWork();
  }
}
