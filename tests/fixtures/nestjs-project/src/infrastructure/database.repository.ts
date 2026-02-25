import { User } from '../domain/user.entity';

export class UserRepository {
  async save(user: User): Promise<void> {
    // Save to DB
  }
}
