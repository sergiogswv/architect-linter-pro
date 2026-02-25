import { Service33 } from '../services/service_33';

export class Controller33 {
  private service = new Service33();

  handle() {
    return this.service.doWork();
  }
}
