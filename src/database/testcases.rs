use super::models;
use super::schema::testcases::dsl;
use crate::repository::testcases::*;
use diesel::pg::Pg;
use diesel::prelude::*;

pub struct TestcaseDB<C: Connection<Backend = Pg>> {
    pub connection: C,
}

impl<C> ITestcaseDB for TestcaseDB<C>
where
    C: Connection<Backend = Pg>,
{
    fn get_all_testcases(&self, testcase_group: &TestcaseGroup) -> Vec<Testcase> {
        dsl::testcases
            .filter(dsl::subject_id.eq(testcase_group.subject_id))
            .filter(dsl::assg.eq(&testcase_group.assignment_id))
            .filter(dsl::ques.eq(&testcase_group.question_id))
            .load::<models::Testcase>(&self.connection)
            .expect("Unable to load testcases.")
            .into_iter()
            .map(|model| Testcase {
                id: model.id,
                group: TestcaseGroup {
                    subject_id: model.subject_id,
                    assignment_id: model.assg,
                    question_id: model.ques,
                },
                content: model.content,
            })
            .collect()
    }

    fn get_users_testcases(&self, user_id: i32) -> Vec<Testcase> {
        dsl::testcases
            .filter(dsl::user_id.eq(user_id))
            .load::<models::Testcase>(&self.connection)
            .expect("Unable to load testcases.")
            .into_iter()
            .map(|model| Testcase {
                id: model.id,
                group: TestcaseGroup {
                    subject_id: model.subject_id,
                    assignment_id: model.assg,
                    question_id: model.ques,
                },
                content: model.content,
            })
            .collect()
    }

    fn delete_testcase(&self, testcase_id: i32, user_id: i32) -> Result<(), DeleteTestcaseError> {
        let model = dsl::testcases
            .find(testcase_id)
            .get_result::<models::Testcase>(&self.connection)
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
            .execute(&self.connection)
            .expect("Unable to delete testcase.");
        Ok(())
    }

    fn save_testcase(&self, user_id: i32, testcase: NewTestcase) {
        let new_testcase = models::NewTestcase {
            user_id,
            subject_id: testcase.group.subject_id,
            assg: testcase.group.assignment_id,
            ques: testcase.group.question_id,
            content: testcase.content,
        };
        diesel::insert_into(dsl::testcases)
            .values(&new_testcase)
            .execute(&self.connection)
            .expect("Unable to insert testcase.");
    }
}
