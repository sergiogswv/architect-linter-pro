import { Service2 } from '../services/service_2';

export class Controller2 {
  private service = new Service2();

  handle() {
    return this.service.doWork();
  }
}
