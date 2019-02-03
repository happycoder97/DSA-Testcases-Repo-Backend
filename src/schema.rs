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
        assg -> Bpchar,
        ques -> Bpchar,
        content -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    submissions,
    testcases,
    users,
);
