table! {
    subjects (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    submissions (id) {
        id -> Int4,
        user_id -> Int4,
        testcase_id -> Int4,
        content -> Text,
        hash1 -> Uuid,
        hash2 -> Uuid,
    }
}

table! {
    testcases (id) {
        id -> Int4,
        user_id -> Int4,
        subject_id -> Int4,
        assignment_char -> Bpchar,
        question_num -> Int4,
        content -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    subjects,
    submissions,
    testcases,
    users,
);
