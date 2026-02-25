import { Service16 } from '../services/service_16';

export class Controller16 {
  private service = new Service16();

  handle() {
    return this.service.doWork();
  }
}
