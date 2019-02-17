use super::RecordError;
use super::User;
use uuid::Uuid;

pub struct NewSubmission {
    pub user_id: i32,
    pub testcase_id: i32,
    pub content: String,
    pub hash1: Uuid,
    pub hash2: Uuid,
}

pub struct Submission {
    pub id: i32,
    pub user_id: i32,
    pub testcase_id: i32,
    pub content: String,
    pub hash1: Uuid,
    pub hash2: Uuid,
}

pub trait ISubmissionDB {
    fn get_by_testcase(&self, testcase_id: i32) -> (Vec<Submission>, Vec<User>);
    fn insert(&self, submission: &NewSubmission);
    fn delete(&self, submission_id: i32) -> Result<(), RecordError>;
}
