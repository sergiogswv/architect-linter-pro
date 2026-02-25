import { Service47 } from '../services/service_47';

export class Controller47 {
  private service = new Service47();

  handle() {
    return this.service.doWork();
  }
}
