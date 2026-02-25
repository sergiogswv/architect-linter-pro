import { Service22 } from '../services/service_22';

export class Controller22 {
  private service = new Service22();

  handle() {
    return this.service.doWork();
  }
}
