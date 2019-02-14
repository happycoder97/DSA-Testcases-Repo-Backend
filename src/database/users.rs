use super::models;
use super::schema::users;
use crate::repository::users::*;
use diesel::pg::Pg;
use diesel::prelude::*;

pub struct UserDB<C: Connection<Backend = Pg>> {
    pub connection: C,
}

impl<C> IUserDB for UserDB<C>
where
    C: Connection<Backend = Pg>,
{
    fn authenticate(&self, username: &str, password: &str) -> Result<User, ()> {
        let results = users::table
            .filter(users::username.eq(username))
            .filter(users::password.eq(password))
            .limit(1)
            .load::<models::User>(&self.connection)
            .unwrap();

        match results.len() {
            0 => return Err(()),
            1 => {}
            _ => panic!("More than one user with same username: {}", username),
        }

        let user_model = &results[0];
        let user = User {
            id: user_model.id,
            username: user_model.username.clone(),
            is_admin: user_model.is_admin,
        };
        Ok(user)
    }

    fn signup(&self, username: String, password: String) {
        let user_model = models::NewUser {
            username,
            password,
            is_admin: false,
        };

        let _user_model: models::User = diesel::insert_into(users::table)
            .values(&user_model)
            .get_result(&self.connection)
            .expect("Error inserting user.");
    }
}
