import { Service7 } from '../services/service_7';

export class Controller7 {
  private service = new Service7();

  handle() {
    return this.service.doWork();
  }
}
