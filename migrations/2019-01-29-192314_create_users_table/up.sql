-- Your SQL goes here
CREATE TABLE UserAccount(
    id SERIAL PRIMARY KEY,
    username VARCHAR(64) UNIQUE NOT NULL,
    password VARCHAR(64) UNIQUE NOT NULL
);