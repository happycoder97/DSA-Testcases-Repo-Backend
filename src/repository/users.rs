pub struct User {
    pub id: i32,
    pub username: String,
    pub is_admin: bool,
}

pub trait IUserDB {
    fn authenticate(&self, username: &str, password: &str) -> Result<User, ()>;
    fn signup(&self, username: String, password: String);
}
