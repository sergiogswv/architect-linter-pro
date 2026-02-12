import { UserRepository } from './user.repository';

/**
 * User Service
 * This is CORRECT - services can use repositories
 */
export class UserService {
  private userRepository: UserRepository;

  constructor() {
    this.userRepository = new UserRepository();
  }

  findUserById(id: number) {
    return this.userRepository.findById(id);
  }

  findAllUsers() {
    return this.userRepository.findAll();
  }

  createUser(data: any) {
    return this.userRepository.create(data);
  }
}
