import { Service40 } from '../services/service_40';

export class Controller40 {
  private service = new Service40();

  handle() {
    return this.service.doWork();
  }
}
