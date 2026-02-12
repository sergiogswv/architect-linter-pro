/**
 * User Repository
 * Data access layer - should not import anything from upper layers
 */
export class UserRepository {
  private users: any[] = [
    { id: 1, name: 'Alice', email: 'alice@example.com' },
    { id: 2, name: 'Bob', email: 'bob@example.com' },
  ];

  findById(id: number) {
    return this.users.find(u => u.id === id);
  }

  findAll() {
    return this.users;
  }

  create(data: any) {
    const newUser = {
      id: this.users.length + 1,
      ...data
    };
    this.users.push(newUser);
    return newUser;
  }

  update(id: number, data: any) {
    const user = this.findById(id);
    if (user) {
      Object.assign(user, data);
    }
    return user;
  }

  delete(id: number) {
    const index = this.users.findIndex(u => u.id === id);
    if (index !== -1) {
      this.users.splice(index, 1);
      return true;
    }
    return false;
  }
}
