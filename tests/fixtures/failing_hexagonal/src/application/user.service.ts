import { User, UserEntity } from '../domain/user.entity';
import { UserRepository } from '../infrastructure/user.repo';

export class UserService {
  constructor(private repo: UserRepository) {}

  createUser(id: string, name: string): User {
    const user: User = { id, name };
    const entity = new UserEntity(user);

    if (entity.validate()) {
      this.repo.save(user);
    }

    return user;
  }
}
