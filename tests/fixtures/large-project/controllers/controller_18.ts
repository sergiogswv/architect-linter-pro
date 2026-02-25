import { Service18 } from '../services/service_18';

export class Controller18 {
  private service = new Service18();

  handle() {
    return this.service.doWork();
  }
}
