use super::models;
use super::schema::testcases::dsl;
use crate::repository::testcases::*;
use diesel::prelude::*;

pub struct TestcaseDB<'a> {
    connection: &'a PgConnection,
}

impl<'a> ITestcaseDB for TestcaseDB<'a> {
    fn get_all_testcases(&self, testcase_group: &TestcaseGroup) -> Vec<String> {
        dsl::testcases
            .filter(dsl::subject_id.eq(testcase_group.subject_id))
            .filter(dsl::assg.eq(&testcase_group.assignment))
            .filter(dsl::ques.eq(&testcase_group.question))
            .load::<models::Testcase>(self.connection)
            .expect("Unable to load testcases.")
            .into_iter()
            .map(|model| model.content)
            .collect()
    }
    fn get_users_testcases(&self, user_id: i32) -> Vec<Testcase> {
        dsl::testcases
            .filter(dsl::user_id.eq(user_id))
            .load::<models::Testcase>(self.connection)
            .expect("Unable to load testcases.")
            .into_iter()
            .map(|model| Testcase {
                id: model.id,
                group: TestcaseGroup {
                    subject_id: model.subject_id,
                    assignment: model.assg,
                    question: model.ques,
                },
                content: model.content,
            })
            .collect()
    }
    fn delete_testcase(&self, testcase_id: i32, user_id: i32) -> Result<(), DeleteTestcaseError> {
        let model = dsl::testcases
            .find(testcase_id)
            .get_result::<models::Testcase>(self.connection)
            .optional()
            .expect("Delete testcase: error querying.");
        if model.is_none() {
            return Err(DeleteTestcaseError::NotFound);
        }
        let model = model.unwrap();
        if model.user_id != user_id {
            return Err(DeleteTestcaseError::NotAuthorized);
        }
        diesel::delete(dsl::testcases.find(testcase_id))
            .execute(self.connection)
            .expect("Unable to delete testcase.");
        Ok(())
    }
}
