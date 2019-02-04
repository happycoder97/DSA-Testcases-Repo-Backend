pub trait ITestcaseDB {
    fn get_all_testcases(
        &self,
        subject_id: i32,
        assignment: String,
        question: String,
    ) -> Vec<String>;
    fn delete_testcase(&self, testcase_id: i32, user_id: i32) -> Result<(), DeleteTestcaseError>;
}

pub enum DeleteTestcaseError {
    NotFound,
    NotAuthorized,
}
