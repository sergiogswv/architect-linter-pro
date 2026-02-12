import { UserService } from './user.service';
import { UserRepository } from './user.repository'; // ❌ VIOLATION: controller -> repository

/**
 * User Controller
 * VIOLATION: This controller imports repository directly
 */
export class UserController {
  private userService: UserService;
  private userRepository: UserRepository; // ❌ Should not use repository directly

  constructor() {
    this.userService = new UserService();
    this.userRepository = new UserRepository(); // ❌ Architectural violation
  }

  getUser(id: number) {
    // Should use service, but also uses repository directly (BAD!)
    const fromService = this.userService.findUserById(id);
    const fromRepo = this.userRepository.findById(id); // ❌ Violation
    return fromRepo || fromService;
  }

  getAllUsers() {
    return this.userRepository.findAll(); // ❌ Bypassing service layer
  }
}
