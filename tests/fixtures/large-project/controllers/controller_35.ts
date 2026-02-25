import { Service35 } from '../services/service_35';

export class Controller35 {
  private service = new Service35();

  handle() {
    return this.service.doWork();
  }
}
