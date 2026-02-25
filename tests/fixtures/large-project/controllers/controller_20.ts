import { Service20 } from '../services/service_20';

export class Controller20 {
  private service = new Service20();

  handle() {
    return this.service.doWork();
  }
}
