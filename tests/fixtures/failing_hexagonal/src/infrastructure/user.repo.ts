import { User } from '../domain/user.entity';

export class UserRepository {
  private db: Map<string, User> = new Map();

  save(user: User): void {
    this.db.set(user.id, user);
  }

  findById(id: string): User | undefined {
    return this.db.get(id);
  }
}
