import { Service38 } from '../services/service_38';

export class Controller38 {
  private service = new Service38();

  handle() {
    return this.service.doWork();
  }
}
