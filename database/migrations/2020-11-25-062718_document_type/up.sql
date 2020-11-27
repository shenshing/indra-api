-- Your SQL goes here
CREATE TABLE IF NOT EXISTS DOCUMENTTYPE (
    ID VARCHAR (36) PRIMARY KEY,
    DOCUMENT_NAME VARCHAR (15),
    CREATED_AT timestamp NOT NULL default current_timestamp,
    UPDATED_AT timestamp
)

