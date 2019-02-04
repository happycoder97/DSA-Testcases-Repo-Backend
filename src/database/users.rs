use super::models;
use super::schema::users;
use diesel::prelude::*;
use crate::repository::users::*;


pub struct UserDB<'conn> {
    connection: &'conn PgConnection,
}

impl<'conn> IUserDB for UserDB<'conn> {
    fn authenticate(&self, username: String, password: String) -> Result<User, ()> {
        let results = users::table
            .filter(users::username.eq(&username))
            .filter(users::password.eq(&password))
            .limit(1)
            .load::<models::User>(self.connection)
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
            .get_result(self.connection)
            .expect("Error inserting user.");
    }
}
