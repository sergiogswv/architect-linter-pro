import { Service41 } from '../services/service_41';

export class Controller41 {
  private service = new Service41();

  handle() {
    return this.service.doWork();
  }
}
