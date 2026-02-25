import { Service14 } from '../services/service_14';

export class Controller14 {
  private service = new Service14();

  handle() {
    return this.service.doWork();
  }
}
