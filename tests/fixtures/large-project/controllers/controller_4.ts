import { Service4 } from '../services/service_4';

export class Controller4 {
  private service = new Service4();

  handle() {
    return this.service.doWork();
  }
}
