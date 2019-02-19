use uuid::Uuid;

/// model_struct is a macro to add implementation defined helpers
/// to the model structs to reduce boilerplate when loading to/from database
/// To be defined by the repository implementation
model_struct! {

    pub struct User, NewUser ["users"] {
        pub username: String,
        pub password: String,
        pub is_admin: bool
    }

    pub struct Subject, NewSubject ["subjects"] {
        pub title: String
    }

    pub struct Testcase, NewTestcase ["testcases"] {
        pub user_id: i32,
        pub subject_id: i32,
        pub assignment_char: String,
        pub question_num: i32,
        pub content: String
    }

    pub struct Submission, NewSubmission ["submissions"] {
        pub user_id: i32,
        pub testcase_id: i32,
        pub content: String,
        pub hash1: Uuid,
        pub hash2: Uuid
    }
}

pub trait CRUD<Model, NewModel> {
    fn create(&self, new_model: &NewModel);
    fn get(&self, id: i32) -> Option<Model>;
    fn get_all(&self) -> Vec<Model>;
    fn update(&self, model: &Model);
    fn delete(&self, id: i32);
}

pub trait IUserDB {
    fn authenticate(&self, username: &str, password: &str) -> Result<User, ()>;
    fn signup(&self, new_user: &NewUser);
    fn change_password(&self, user_id: i32, password: &str);
}

pub trait ISubjectDB: CRUD<Subject, NewSubject> {}

pub struct TestcaseGroup {
    pub subject_id: i32,
    pub assignment_char: String,
    pub question_num: i32,
}

pub trait ITestcaseDB: CRUD<Testcase, NewTestcase> {
    fn get_by_group(&self, testcase_group: &TestcaseGroup) -> (Vec<Testcase>, Vec<User>);
    fn get_by_user(&self, username: &str) -> Vec<Testcase>;
}

pub trait ISubmissionDB: CRUD<Submission, NewSubmission> {
    fn create_or_update(&self, submission: &NewSubmission);
    fn get_by_testcase(&self, testcase_id: i32) -> (Vec<Submission>, Vec<User>);
}
