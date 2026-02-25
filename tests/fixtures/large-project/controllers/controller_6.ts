import { Service6 } from '../services/service_6';

export class Controller6 {
  private service = new Service6();

  handle() {
    return this.service.doWork();
  }
}
