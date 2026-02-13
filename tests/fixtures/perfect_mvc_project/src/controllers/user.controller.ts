import { UserModel } from '../models/user.model';
import { UserView } from '../views/user.view';
import { User } from '../models/user.model';

export class UserController {
  constructor(
    private model: UserModel,
    private view: UserView
  ) {}

  addUser(id: string, name: string, email: string): void {
    const user: User = { id, name, email };
    this.model.add(user);
  }

  displayUser(id: string): string {
    const user = this.model.findById(id);
    if (user) {
      return this.view.display(user);
    }
    return 'User not found';
  }
}
