from ..models.user import User

class UserService:
    def create_user(self, name):
        return User('1', name)
