export class UserService {
    constructor(private id: number) {}

    getUser(): User {
        return new User(this.id);
    }

    updateUser(user: User): void {
        console.log(`Updating user ${user.id}`);
    }
}

export class User {
    constructor(public id: number) {}
}
