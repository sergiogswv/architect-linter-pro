import { User } from '../entities/user.entity';
import { UserUseCase } from '../usecases/user.usecase';

export class UserAdapter {
  constructor(private useCase: UserUseCase) {}

  async createUserFromAPI(apiData: unknown): Promise<User> {
    const data = apiData as { id: string; name: string; email: string };
    return this.useCase.createUser(data.id, data.name, data.email);
  }
}
