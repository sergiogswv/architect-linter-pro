import { Service25 } from '../services/service_25';

export class Controller25 {
  private service = new Service25();

  handle() {
    return this.service.doWork();
  }
}
