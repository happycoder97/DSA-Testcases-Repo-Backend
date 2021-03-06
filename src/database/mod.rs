use diesel::pg::Pg;
use diesel::prelude::*;

use crate::repository::*;

pub mod schema;

#[macro_use]
pub mod model_struct;

pub mod users;

pub struct Database<C: Connection<Backend = Pg>> {
    pub connection: C,
}

#[macro_use]
mod impl_crud;

impl_crud!(Database<Pg>, Subject, NewSubject, schema::subjects::table);
impl<C> ISubjectDB for Database<C> where C: Connection<Backend = Pg> {}

impl_crud!(
    Database<Pg>,
    Testcase,
    NewTestcase,
    schema::testcases::table
);

impl<C> ITestcaseDB for Database<C>
where
    C: Connection<Backend = Pg>,
{
    fn get_by_group(&self, testcase_group: &TestcaseGroup) -> (Vec<Testcase>, Vec<User>) {
        use schema::testcases::dsl;
        let testcases = dsl::testcases
            .filter(dsl::subject_id.eq(testcase_group.subject_id))
            .filter(dsl::assignment_char.eq(&testcase_group.assignment_char))
            .filter(dsl::question_num.eq(&testcase_group.question_num))
            .load::<Testcase>(&self.connection)
            .expect("Unable to load testcases.")
            .into_iter()
            .collect();

        use schema::users::dsl as users_dsl;
        let users = users_dsl::users
            .filter(
                users_dsl::id.eq_any(
                    dsl::testcases
                        .filter(dsl::subject_id.eq(testcase_group.subject_id))
                        .filter(dsl::assignment_char.eq(&testcase_group.assignment_char))
                        .filter(dsl::question_num.eq(&testcase_group.question_num))
                        .select(dsl::user_id),
                ),
            )
            .load::<User>(&self.connection)
            .expect("Unable to get users for submissions.")
            .into_iter()
            .collect();
        (testcases, users)
    }

    fn get_by_user(&self, username: &str) -> Vec<Testcase> {
        use schema::testcases::dsl;
        use schema::users::dsl as users_dsl;
        dsl::testcases
            .filter(
                dsl::user_id.eq_any(
                    users_dsl::users
                        .filter(users_dsl::username.eq(username))
                        .select(users_dsl::id),
                ),
            )
            .load::<Testcase>(&self.connection)
            .expect("Unable to load testcases.")
            .into_iter()
            .collect()
    }
}

impl_crud!(
    Database<Pg>,
    Submission,
    NewSubmission,
    schema::submissions::table
);

impl<C> ISubmissionDB for Database<C>
where
    C: Connection<Backend = Pg>,
{
    fn get_by_testcase(&self, testcase_id: i32) -> (Vec<Submission>, Vec<User>) {
        use schema::submissions::dsl;
        let submissions = dsl::submissions
            .filter(dsl::testcase_id.eq(testcase_id))
            .load::<Submission>(&self.connection)
            .expect("Unable to get_by_testcase submissions.")
            .into_iter()
            .collect();

        use schema::users::dsl as users_dsl;
        let users = users_dsl::users
            .filter(
                users_dsl::id.eq_any(
                    dsl::submissions
                        .filter(dsl::testcase_id.eq(testcase_id))
                        .select(dsl::user_id),
                ),
            )
            .load::<User>(&self.connection)
            .expect("Unable to get users for submissions.")
            .into_iter()
            .collect();

        (submissions, users)
    }

    fn create_or_update(&self, submission: &NewSubmission) {
        use schema::submissions::dsl;
        let mut submissions: Vec<Submission> = dsl::submissions
            .filter(dsl::testcase_id.eq(submission.testcase_id))
            .filter(dsl::user_id.eq(submission.user_id))
            .load::<Submission>(&self.connection)
            .expect("Unable to get for create_or_update submissions.")
            .into_iter()
            .collect();

        if let Some(old_submission) = submissions.pop() {
            let mut new_submission = old_submission;
            new_submission.content = submission.content.clone();
            new_submission.hash1 = submission.hash1;
            new_submission.hash2 = submission.hash2;
            self.update(&new_submission);
        } else {
            self.create(submission);
        }
    }
}
