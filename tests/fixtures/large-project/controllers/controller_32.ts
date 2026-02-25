import { Service32 } from '../services/service_32';

export class Controller32 {
  private service = new Service32();

  handle() {
    return this.service.doWork();
  }
}
