import { Service26 } from '../services/service_26';

export class Controller26 {
  private service = new Service26();

  handle() {
    return this.service.doWork();
  }
}
