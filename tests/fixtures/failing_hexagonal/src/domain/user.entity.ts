// This violates hexagonal architecture by importing from infrastructure!
import { UserRepository } from '../infrastructure/user.repo';

export interface User {
  id: string;
  name: string;
}

export class UserEntity {
  constructor(private user: User) {}

  validate(): boolean {
    return this.user.name.length > 0;
  }

  // VIOLATION: Domain should not know about infrastructure
  save(repo: UserRepository): void {
    repo.save(this.user);
  }
}
