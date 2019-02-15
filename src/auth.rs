use crate::repository::users::IUserDB;
use base64;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

pub struct UserGuard {
    pub id: i32,
}

pub struct AdminGuard {
    pub id: i32,
}

#[derive(Debug)]
pub enum AuthError {
    AuthFailed,
    PrivilegeError,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, AuthError> {
        let user_db: Box<dyn IUserDB> = request.guard::<Box<dyn IUserDB>>().unwrap();
        let user_db = user_db.as_ref();

        let user = request
            .headers()
            .get_one("Authentication")
            .and_then(extract_username_password_from_b64)
            .and_then(|(username, password)| user_db.authenticate(&username, &password).ok());

        match user {
            Some(user) => Outcome::Success(UserGuard { id: user.id }),
            _ => Outcome::Failure((Status::Forbidden, AuthError::AuthFailed)),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminGuard {
    type Error = AuthError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, AuthError> {
        let user_db: Box<dyn IUserDB> = request.guard::<Box<dyn IUserDB>>().unwrap();
        let user_db = user_db.as_ref();

        let user = request
            .headers()
            .get_one("Authentication")
            .and_then(extract_username_password_from_b64)
            .and_then(|(username, password)| user_db.authenticate(&username, &password).ok())
            .filter(|user| user.is_admin);

        match user {
            Some(user) => Outcome::Success(AdminGuard { id: user.id }),
            _ => Outcome::Failure((Status::Forbidden, AuthError::AuthFailed)),
        }
    }
}

fn extract_username_password_from_b64(header_value: &str) -> Option<(String, String)> {
    let b64_auth = if header_value.starts_with("Basic ") {
        Some(header_value["Basic ".len()..].trim())
    } else {
        None
    };
    let auth_str = b64_auth
        .as_ref()
        .and_then(|b64_auth| base64::decode(b64_auth).ok())
        .and_then(|decoded_vec| String::from_utf8(decoded_vec).ok());

    auth_str.and_then(|auth| {
        let mut split_by_colon = auth.split(':');
        let username = split_by_colon.next().map(String::from);
        let password = split_by_colon.next().map(String::from);
        if let (Some(username), Some(password)) = (username, password) {
            Some((username, password))
        } else {
            None
        }
    })
}
