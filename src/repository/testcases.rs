use super::RecordError;
use super::User;

pub struct TestcaseGroup {
    pub subject_id: i32,
    pub assignment_char: String,
    pub question_num: i32,
}

pub struct Testcase {
    pub id: i32,
    pub user_id: i32,
    pub group: TestcaseGroup,
    pub content: String,
}

pub struct NewTestcase {
    pub group: TestcaseGroup,
    pub content: String,
}

pub trait ITestcaseDB {
    fn get_by_group(&self, testcase_group: &TestcaseGroup) -> (Vec<Testcase>, Vec<User>);
    fn get_by_user(&self, user_id: i32) -> Vec<Testcase>;
    fn insert(&self, user_id: i32, testcase: NewTestcase);
    fn try_delete(&self, testcase_id: i32, user_id: i32) -> Result<(), RecordError>;
    fn try_update(
        &self,
        testcase_id: i32,
        user_id: i32,
        testcase: &NewTestcase,
    ) -> Result<(), RecordError>;
}
