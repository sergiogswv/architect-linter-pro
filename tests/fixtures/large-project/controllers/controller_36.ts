import { Service36 } from '../services/service_36';

export class Controller36 {
  private service = new Service36();

  handle() {
    return this.service.doWork();
  }
}
