use crate::DBConn;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use crate::database::{
    subjects::SubjectDB, submissions::SubmissionDB, testcases::TestcaseDB, users::UserDB,
};
use crate::repository::{
    subjects::ISubjectDB, submissions::ISubmissionDB, testcases::ITestcaseDB, users::IUserDB,
};

type Connection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
macro_rules! impl_from_request {
    ($trait_name: ident, $struct_name: ident) => {
        impl<'a, 'r> FromRequest<'a, 'r> for Box<dyn $trait_name> {
            type Error = ();

            fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
                let connection = request.guard::<DBConn>().unwrap().0;
                let b: Box<$struct_name<Connection>> = Box::from($struct_name {
                    connection: connection,
                });
                Outcome::Success(b)
            }
        }
    };
}

// Implement FromRequest<Box<IXyzDB>>
impl_from_request!(ISubjectDB, SubjectDB);
impl_from_request!(ISubmissionDB, SubmissionDB);
impl_from_request!(ITestcaseDB, TestcaseDB);
impl_from_request!(IUserDB, UserDB);
