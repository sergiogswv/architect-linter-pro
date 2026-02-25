import { Service46 } from '../services/service_46';

export class Controller46 {
  private service = new Service46();

  handle() {
    return this.service.doWork();
  }
}
