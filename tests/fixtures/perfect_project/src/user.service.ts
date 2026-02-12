/**
 * User Service - business logic layer
 * Clean service with short, focused methods
 */
export class UserService {
  private users: any[] = [];

  findUserById(id: number) {
    return this.users.find(u => u.id === id);
  }

  findAllUsers() {
    return this.users;
  }

  createUser(data: { name: string; email: string }) {
    const newUser = {
      id: this.users.length + 1,
      ...data
    };
    this.users.push(newUser);
    return newUser;
  }

  updateUser(id: number, data: any) {
    const user = this.findUserById(id);
    if (user) {
      Object.assign(user, data);
      return user;
    }
    return null;
  }

  deleteUser(id: number) {
    const index = this.users.findIndex(u => u.id === id);
    if (index !== -1) {
      this.users.splice(index, 1);
      return true;
    }
    return false;
  }
}
