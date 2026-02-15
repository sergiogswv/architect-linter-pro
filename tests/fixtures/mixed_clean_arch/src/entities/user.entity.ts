import { UserUseCase } from '../usecases/user.usecase';

export interface User {
  id: string;
  name: string;
  email: string;
}

export class UserEntity {
  constructor(private user: User, private uc: UserUseCase) {}

  isValid(): boolean {
    return this.user.email.includes('@');
  }
}
