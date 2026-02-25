import { Service30 } from '../services/service_30';

export class Controller30 {
  private service = new Service30();

  handle() {
    return this.service.doWork();
  }
}
