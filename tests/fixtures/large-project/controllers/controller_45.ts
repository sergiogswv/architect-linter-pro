import { Service45 } from '../services/service_45';

export class Controller45 {
  private service = new Service45();

  handle() {
    return this.service.doWork();
  }
}
