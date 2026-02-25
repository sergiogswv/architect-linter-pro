import { Service27 } from '../services/service_27';

export class Controller27 {
  private service = new Service27();

  handle() {
    return this.service.doWork();
  }
}
