-- Your SQL goes here
CREATE TABLE IF NOT EXISTS DOCUMENTS (
    ID VARCHAR (36) PRIMARY KEY,
    DOCUMENTS_NO VARCHAR (50),
    DOCUMENTTYPE_ID VARCHAR (50),
    DOCUMENT_URI TEXT NOT NULL,
    FACE_URI TEXT NOT NULL,
    ISSUE_DATE TEXT NOT NULL,
    EXPIRE_DATE TEXT NOT NULL,
    CREATED_AT TIMESTAMP NOT NULL default current_timestamp,
    UPDATED_AT TIMESTAMP,
    CREATED_BY VARCHAR (36) UNIQUE,
    UPDATED_BY VARCHAR (36)
)

