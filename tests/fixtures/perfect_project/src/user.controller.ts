import { UserService } from './user.service';

/**
 * User Controller - handles HTTP requests
 * This is a clean, short controller with no violations
 */
export class UserController {
  private userService: UserService;

  constructor() {
    this.userService = new UserService();
  }

  getUser(id: number) {
    return this.userService.findUserById(id);
  }

  getAllUsers() {
    return this.userService.findAllUsers();
  }

  createUser(name: string, email: string) {
    return this.userService.createUser({ name, email });
  }

  updateUser(id: number, data: any) {
    return this.userService.updateUser(id, data);
  }

  deleteUser(id: number) {
    return this.userService.deleteUser(id);
  }
}
