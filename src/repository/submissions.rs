use uuid::Uuid;

pub struct NewSubmission {
    pub user_id: i32, 
    pub testcase_id: i32,
    pub content: String
}

pub struct SavedSubmission {
    pub id: i32,
    pub user_id: i32, 
    pub testcase_id: i32,
    pub content: String,
    pub hash1: Uuid,
    pub hash2: Uuid
}


pub trait ISubmissionDB {
    fn get_all(&self, testcase_id: int) -> Vec<SavedSubmission>;
    fn submit(&self, submission: &NewSubmission);
    fn delete(&self, submission_id: i32, user_id: i32) -> Result<(), DeleteSubmissionError>;
}

pub enum DeleteSubmissionError {
    NotFound,
    NotAuthorized,
}