-- Your SQL goes here
CREATE TABLE IF NOT EXISTS STATUS (
    ID VARCHAR (36) PRIMARY KEY,
    STATUS_NAME VARCHAR (10),
    CREATED_AT timestamp NOT NULL default current_timestamp,
    UPDATED_AT timestamp
)