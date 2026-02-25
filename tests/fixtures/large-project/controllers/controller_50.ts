import { Service50 } from '../services/service_50';

export class Controller50 {
  private service = new Service50();

  handle() {
    return this.service.doWork();
  }
}
