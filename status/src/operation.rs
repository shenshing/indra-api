use std::time::SystemTime;

use diesel::PgConnection;
use diesel::prelude::*;


#[derive(Queryable, Debug)]
pub struct Status {
    id: String, 
    status_name: Option<String>,
    created_at: SystemTime,
    updated_at: Option<SystemTime>,
}
pub fn get_status_by_name(status_: String, connection: &PgConnection) -> Result<Status, String> {
    use database::schema::status::dsl::{status_name, status};
    match status.filter(status_name.eq(status_))
        .get_result::<Status>(connection) {
            Ok(status_) => return Ok(status_),
            Err(err) => return Err(format!("Error: {}", err)) 
        }
}