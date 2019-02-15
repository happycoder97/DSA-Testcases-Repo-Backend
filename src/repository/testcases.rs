pub struct TestcaseGroup {
    pub subject_id: i32,
    pub assignment_id: String,
    pub question_id: String,
}

pub struct Testcase {
    pub id: i32,
    pub group: TestcaseGroup,
    pub content: String,
}

pub struct NewTestcase {
    pub group: TestcaseGroup,
    pub content: String,
}

pub trait ITestcaseDB {
    fn get_all_testcases(&self, testcase_group: &TestcaseGroup) -> Vec<Testcase>;
    fn get_users_testcases(&self, user_id: i32) -> Vec<Testcase>;
    fn delete_testcase(&self, testcase_id: i32, user_id: i32) -> Result<(), DeleteTestcaseError>;
    fn save_testcase(&self, user_id: i32, testcase: NewTestcase);
}

pub enum DeleteTestcaseError {
    NotFound,
    NotAuthorized,
}
