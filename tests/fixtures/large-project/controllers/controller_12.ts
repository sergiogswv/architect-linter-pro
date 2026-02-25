import { Service12 } from '../services/service_12';

export class Controller12 {
  private service = new Service12();

  handle() {
    return this.service.doWork();
  }
}
