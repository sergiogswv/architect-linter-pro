import { User } from '../models/user.model';

export class UserView {
  display(user: User): string {
    return `User: ${user.name} (${user.email})`;
  }

  displayList(users: User[]): string {
    return users.map(u => this.display(u)).join('\n');
  }
}
