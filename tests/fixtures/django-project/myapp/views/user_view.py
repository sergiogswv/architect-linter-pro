from ..services.user_service import UserService

class UserView:
    def __init__(self):
        self.service = UserService()

    def create(self, name):
        return self.service.create_user(name)
