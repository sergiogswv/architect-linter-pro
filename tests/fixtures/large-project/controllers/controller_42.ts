import { Service42 } from '../services/service_42';

export class Controller42 {
  private service = new Service42();

  handle() {
    return this.service.doWork();
  }
}
