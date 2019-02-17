pub struct User {
    pub user_id: i32,
    pub username: String,
    pub is_admin: bool,
}

pub struct NewUser {
    pub user_id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

pub trait IUserDB {
    fn authenticate(&self, username: &str, password: &str) -> Result<User, ()>;
    fn signup(&self, new_user: &NewUser);
}
