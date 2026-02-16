class UserService:
    def __init__(self, user_id: int):
        self.user_id = user_id

    def get_user(self) -> dict:
        return {"id": self.user_id}

    def update_user(self, user: dict) -> None:
        print(f"Updating user {user['id']}")
