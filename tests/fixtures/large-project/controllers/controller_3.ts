import { Service3 } from '../services/service_3';

export class Controller3 {
  private service = new Service3();

  handle() {
    return this.service.doWork();
  }
}
