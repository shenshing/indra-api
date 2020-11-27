use std::time::SystemTime;
use database::schema::apiacc;

use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct ApiAccount {
    pub id: String,
    pub apikey: Option<String>,
    pub apisec: Option<String>,
    pub is_active: bool,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub updated_by: Option<String>
}

#[derive(Insertable)]
#[table_name="apiacc"]
pub struct Api {
    pub id: String, 
    pub apikey: String,
    pub apisec: String,
    // pub updated_at: SystemTime,
    // pub updated_by: String
}

pub fn insert_api_key(new_key: Api, connection: &PgConnection) -> Result<String, String> {
    match diesel::insert_into(apiacc::table)
        .values(&new_key)
        .execute(connection) {
            Ok(_) => return Ok(format!("insert new key successful")),
            Err(err) => return Err(format!("error saving new wallet {}", err))
        }
}

pub fn get_acc_by_apikey(apikey_: String, connection: &PgConnection) -> Result<ApiAccount, String> {
    use database::schema::apiacc::dsl::{apiacc, apikey};
    match apiacc.filter(apikey.eq(apikey_))
        .get_result::<ApiAccount>(connection) {
            Ok(acc) => return Ok(acc),
            Err(err) => return Err(format!("Error: {}", err))
        }
}