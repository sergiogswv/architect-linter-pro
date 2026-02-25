import { Service44 } from '../services/service_44';

export class Controller44 {
  private service = new Service44();

  handle() {
    return this.service.doWork();
  }
}
