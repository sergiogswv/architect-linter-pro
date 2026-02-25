import { Service43 } from '../services/service_43';

export class Controller43 {
  private service = new Service43();

  handle() {
    return this.service.doWork();
  }
}
