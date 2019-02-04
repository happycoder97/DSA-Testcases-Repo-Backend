use super::schema::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub is_admin: bool,
}

#[derive(Queryable)]
pub struct Subject {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "subjects"]
pub struct NewSubject {
    pub title: String,
}

#[derive(Queryable)]
pub struct Testcase {
    pub id: i32,
    pub user_id: i32,
    pub subject_id: i32,
    pub assg: String,
    pub ques: String,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "testcases"]
pub struct NewTestcase {
    pub user_id: i32,
    pub subject_id: i32,
    pub assg: String,
    pub ques: String,
    pub content: String,
}

#[derive(Queryable)]
pub struct Submission {
    pub id: i32,
    pub user_id: i32,
    pub testcase_id: i32,
    pub content: String,
    pub hash1: Uuid,
    pub hash2: Uuid,
}

#[derive(Insertable)]
#[table_name = "submissions"]
pub struct NewSubmission {
    pub user_id: i32,
    pub testcase_id: i32,
    pub content: String,
    pub hash1: Uuid,
    pub hash2: Uuid,
}
