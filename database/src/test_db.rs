use std::time::SystemTime;

use diesel::prelude::*;
use diesel::PgConnection;
use crate::schema::test;

#[derive(Insertable, Queryable)]
#[table_name="test"]
pub struct Test {
    id:             i32,
    password:       String,
    description:    Option<String>,
    active:         Option<bool>,
    item:           Option<String>,
    updated:        Option<SystemTime>,
}


pub fn insert_new_value(connection: &PgConnection, new_test: Test) {
    diesel::insert_into(test::table)
        .values(&new_test)
        .execute(connection)
        .expect("Error saving new post");   
}