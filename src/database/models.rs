use super::schema::*;
use uuid::Uuid;

macro_rules! diesel_struct {
    (
        $(
            pub struct $struct_name:ident, $new_struct_name: ident [$table_name: literal]{
                $(pub $field:ident : $type:ty,)*
            }
        )+
    ) => (
        $(
            #[derive(Queryable)]
            #[table_name = $table_name]
            pub struct $struct_name {
                pub id: i32,
                $(pub $field: $type),*
            }

            #[derive(Insertable)]
            #[table_name = $table_name]
            pub struct $new_struct_name {
                $(pub $field: $type),*
            }
        )+
    )
}

diesel_struct! {
    pub struct User, NewUser ["users"] {
        pub username: String,
        pub password: String,
        pub is_admin: bool,
    }

    pub struct Subject, NewSubject ["subjects"] {
        pub title: String,
    }

    pub struct Testcase, NewTestcase ["testcases"] {
        pub user_id: i32,
        pub subject_id: i32,
        pub assignment_char: String,
        pub question_num: i32,
        pub content: String,
    }

    pub struct Submission, NewSubmission ["submissions"] {
        pub user_id: i32,
        pub testcase_id: i32,
        pub content: String,
        pub hash1: Uuid,
        pub hash2: Uuid,
    }
}
