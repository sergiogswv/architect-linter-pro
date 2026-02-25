import { Service48 } from '../services/service_48';

export class Controller48 {
  private service = new Service48();

  handle() {
    return this.service.doWork();
  }
}
