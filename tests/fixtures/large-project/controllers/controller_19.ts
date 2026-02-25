import { Service19 } from '../services/service_19';

export class Controller19 {
  private service = new Service19();

  handle() {
    return this.service.doWork();
  }
}
