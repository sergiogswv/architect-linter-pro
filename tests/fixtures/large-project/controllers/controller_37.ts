import { Service37 } from '../services/service_37';

export class Controller37 {
  private service = new Service37();

  handle() {
    return this.service.doWork();
  }
}
