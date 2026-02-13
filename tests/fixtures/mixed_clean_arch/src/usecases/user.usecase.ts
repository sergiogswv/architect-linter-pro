import { User, UserEntity } from '../entities/user.entity';

export class UserUseCase {
  private users: User[] = [];

  createUser(id: string, name: string, email: string): User {
    const user: User = { id, name, email };
    const entity = new UserEntity(user);

    if (!entity.isValid()) {
      throw new Error('Invalid user');
    }

    this.users.push(user);
    return user;
  }

  // This function is intentionally slightly long for testing
  processUserData(
    id: string,
    name: string,
    email: string,
    validate: boolean,
    save: boolean,
    notify: boolean
  ): User {
    const user: User = { id, name, email };

    if (validate) {
      const entity = new UserEntity(user);
      if (!entity.isValid()) {
        throw new Error('Invalid user');
      }
    }

    if (save) {
      this.users.push(user);
    }

    if (notify) {
      console.log(`User created: ${user.name}`);
    }

    return user;
  }
}
