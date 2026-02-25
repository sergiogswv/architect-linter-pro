import { Service39 } from '../services/service_39';

export class Controller39 {
  private service = new Service39();

  handle() {
    return this.service.doWork();
  }
}
