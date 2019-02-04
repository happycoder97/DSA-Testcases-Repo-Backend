pub struct TestcaseGroup {
    pub subject_id: i32,
    pub assignment: String,
    pub question: String,
}

pub struct Testcase {
    pub id: i32,
    pub group: TestcaseGroup,
    pub content: String,
}

pub trait ITestcaseDB {
    fn get_all_testcases(&self, testcase_group: &TestcaseGroup) -> Vec<String>;
    fn get_users_testcases(&self, user_id: i32) -> Vec<Testcase>;
    fn delete_testcase(&self, testcase_id: i32, user_id: i32) -> Result<(), DeleteTestcaseError>;
}

pub enum DeleteTestcaseError {
    NotFound,
    NotAuthorized,
}
