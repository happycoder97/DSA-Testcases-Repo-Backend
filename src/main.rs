#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use uuid::Uuid;

mod auth;
#[macro_use]
mod database;
mod db_cfg;
mod repository;

use auth::{AdminGuard, UserGuard};
use repository::*;

//  Database connection --------------------

use diesel::PgConnection;

#[database("dsa_repo")]
struct DBConn(PgConnection);

// Main ------------------------------------

fn main() {
    rocket::ignite()
        .attach(DBConn::fairing())
        .mount(
            "/",
            routes![
                home,
                login_check,
                get_subject,
                put_subject,
                post_subject,
                delete_subject,
                get_testcase_by_group,
                get_testcase_by_user,
                post_testcase,
                put_submission,
                get_submission,
            ],
        )
        .launch();
}

// Routes ----------------------------------

#[get("/")]
fn home() -> &'static str {
    return concat!(
        "Testcases repository for our lab courses by CSE 17-21 Batch, NIT Calicut",
        "<hr>",
        "<pre>
            * Authorization: basic
            *
            * /login_check
            * Check if authorization is okay
            *
            * /change_password
            * POST: Update password
            *
            * /subject
            * [admin only]
            * POST: New subject
            * PUT: Update subject
            * DELETE: Delete subject
            *
            * /subject
            * GET: Get all
            *
            * /testcase
            * POST: New testcase
            *
            * /testcase?subject=<sub>&assignment_char=<assg>&question_num=<q_num>
            * GET: Get all testcases
            *
            * /testcase?id=<id>
            * [if posted by same user]
            * DELETE: delete
            * PUT: Update
            *
            * /testcase?username=<username>
            * GET: Get all testcases
            *
            * /submission
            * PUT: Set submission of a testcase
            *
            * /submission?testcase_id=<id>
            * GET: Get submissions for a testcase
            *
        </pre>"
    );
}

// User -----------------------------------

#[derive(Serialize)]
struct LoginCheck {
    success: bool,
    id: Option<i32>,
    is_admin: Option<bool>,
}

#[get("/login_check")]
fn login_check(user: Option<UserGuard>, admin: Option<AdminGuard>) -> Json<LoginCheck> {
    if let Some(admin) = admin {
        Json(LoginCheck {
            success: true,
            id: Some(admin.id),
            is_admin: Some(true),
        })
    } else if let Some(user) = user {
        Json(LoginCheck {
            success: true,
            id: Some(user.id),
            is_admin: Some(false),
        })
    } else {
        Json(LoginCheck {
            success: false,
            id: None,
            is_admin: None,
        })
    }
}

// Subject --------------------------------
#[get("/subject")]
fn get_subject(subject_db: Box<dyn ISubjectDB>) -> Json<Vec<Subject>> {
    Json(subject_db.get_all())
}

#[post("/subject", data = "<subject>")]
fn post_subject(_admin: AdminGuard, subject: Json<NewSubject>, subject_db: Box<dyn ISubjectDB>) {
    subject_db.create(&subject);
}

#[put("/subject", data = "<subject>")]
fn put_subject(_admin: AdminGuard, subject: Json<Subject>, subject_db: Box<dyn ISubjectDB>) {
    let subject = subject.into_inner();
    subject_db.update(&subject);
}

#[delete("/subject", data = "<subject_id>")]
fn delete_subject(_admin: AdminGuard, subject_id: Json<(i32)>, subject_db: Box<dyn ISubjectDB>) {
    let subject_id = subject_id.into_inner();
    subject_db.delete(subject_id);
}

// Testcases --------------------------------

#[derive(Deserialize)]
struct TestcaseForm {
    subject_id: i32,
    assignment_char: String,
    question_num: i32,
    content: String,
}

#[derive(Serialize)]
struct UserViewModel {
    id: i32,
    username: String,
}

#[get("/testcase?<subject_id>&<assignment_char>&<question_num>", rank = 1)]
fn get_testcase_by_group(
    subject_id: i32,
    assignment_char: String,
    question_num: i32,
    testcase_db: Box<dyn ITestcaseDB>,
) -> Json<(Vec<Testcase>, Vec<UserViewModel>)> {
    let (testcases, users) = testcase_db.get_by_group(&TestcaseGroup {
        subject_id,
        assignment_char,
        question_num,
    });
    let testcase_users = users
        .into_iter()
        .map(|user| UserViewModel {
            id: user.id,
            username: user.username,
        })
        .collect();
    Json((testcases, testcase_users))
}

#[get("/testcase?<username>", rank = 2)]
fn get_testcase_by_user(
    username: String,
    testcase_db: Box<dyn ITestcaseDB>,
) -> Json<Vec<Testcase>> {
    let testcases = testcase_db.get_by_user(&username);
    Json(testcases)
}

#[post("/testcase", data = "<testcase>")]
fn post_testcase(user: UserGuard, testcase: Json<TestcaseForm>, testcase_db: Box<dyn ITestcaseDB>) {
    let testcase: TestcaseForm = testcase.into_inner();
    testcase_db.create(&NewTestcase {
        user_id: user.id,
        subject_id: testcase.subject_id,
        assignment_char: testcase.assignment_char,
        question_num: testcase.question_num,
        content: testcase.content,
    });
}

// Testcases --------------------------------

#[derive(Deserialize)]
struct SubmissionForm {
    pub testcase_id: i32,
    pub content: String,
}

#[put("/submission", data = "<submission>")]
fn put_submission(
    user: UserGuard,
    submission: Json<SubmissionForm>,
    submission_db: Box<dyn ISubmissionDB>,
) {
    let submission = submission.into_inner();
    let (hash1, hash2) = get_hash(&submission.content);
    submission_db.create_or_update(&NewSubmission {
        user_id: user.id,
        testcase_id: submission.testcase_id,
        content: submission.content,
        hash1,
        hash2,
    });
}

#[get("/submission?<testcase_id>")]
fn get_submission(
    testcase_id: i32,
    submission_db: Box<dyn ISubmissionDB>,
) -> Json<(Vec<Submission>, Vec<UserViewModel>)> {
    let (submissions, users) = submission_db.get_by_testcase(testcase_id);
    let user_views = users
        .into_iter()
        .map(|user| UserViewModel {
            id: user.id,
            username: user.username,
        })
        .collect();
    Json((submissions, user_views))
}

fn get_hash(content: &str) -> (Uuid, Uuid) {
    let mut content1 = String::new();
    for line in content.lines() {
        content1 += line.trim();
        content1.push('\n');
    }
    let hash1 = md5::compute(content1);
    let content2 = content.replace(" ", "").replace("\n", "");
    let hash2 = md5::compute(content2);

    // Note: Diesel forces me to use uuid crate version 0.6
    //       v0.7 provides from slice method
    //       I don't know if `from_random_bytes` care where those bytes come from.
    let hash1 = Uuid::from_random_bytes(hash1.0);
    let hash2 = Uuid::from_random_bytes(hash2.0);
    (hash1, hash2)
}
