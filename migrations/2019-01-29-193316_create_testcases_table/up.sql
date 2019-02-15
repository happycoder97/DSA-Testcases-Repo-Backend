-- Your SQL goes here
CREATE TABLE testcases(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    subject_id INTEGER NOT NULL,
    assignment_char CHAR(5) NOT NULL,
    question_num INTEGER NOT NULL,
    content TEXT NOT NULL
);
