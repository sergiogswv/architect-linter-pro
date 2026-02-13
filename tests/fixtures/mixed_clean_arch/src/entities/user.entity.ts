export interface User {
  id: string;
  name: string;
  email: string;
}

export class UserEntity {
  constructor(private user: User) {}

  isValid(): boolean {
    return this.user.email.includes('@');
  }
}
