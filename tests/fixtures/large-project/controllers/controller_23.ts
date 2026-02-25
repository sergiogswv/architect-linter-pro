import { Service23 } from '../services/service_23';

export class Controller23 {
  private service = new Service23();

  handle() {
    return this.service.doWork();
  }
}
