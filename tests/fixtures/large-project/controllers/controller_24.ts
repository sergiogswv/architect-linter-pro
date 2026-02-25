import { Service24 } from '../services/service_24';

export class Controller24 {
  private service = new Service24();

  handle() {
    return this.service.doWork();
  }
}
