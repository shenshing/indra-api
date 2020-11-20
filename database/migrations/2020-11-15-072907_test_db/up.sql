-- Your SQL goes here
Create Table Test (
    id      Serial Primary Key,
    password    Varchar(256) Not Null,
    description Text,
    active  Boolean,
    item    Varchar(50) UNIQUE,
    updated Timestamp
)