use crate::DBConn;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

use crate::database::Database;
use crate::repository::{ISubjectDB, ISubmissionDB, ITestcaseDB, IUserDB};

type Connection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
macro_rules! impl_from_request {
    ($trait_name: ident) => {
        impl<'a, 'r> FromRequest<'a, 'r> for Box<dyn $trait_name> {
            type Error = ();

            fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
                let connection = request.guard::<DBConn>().unwrap().0;
                let b: Box<Database<Connection>> = Box::from(Database {
                    connection: connection,
                });
                Outcome::Success(b)
            }
        }
    };
}

impl_from_request!(ISubjectDB);
impl_from_request!(ISubmissionDB);
impl_from_request!(ITestcaseDB);
impl_from_request!(IUserDB);
