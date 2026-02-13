export interface User {
  id: string;
  name: string;
  email: string;
}

export class UserModel {
  private users: User[] = [];

  add(user: User): void {
    this.users.push(user);
  }

  findById(id: string): User | undefined {
    return this.users.find(u => u.id === id);
  }
}
