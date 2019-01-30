-- Your SQL goes here
CREATE TABLE Submission(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL, 
    testcase_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    hash1 UUID NOT NULL,
    hash2 UUID NOT NULL
);