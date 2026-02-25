import { User } from '../domain/user.entity';

export class UserService {
  constructor() {}

  createUser(name: string): User {
    return new User('1', name);
  }
}
