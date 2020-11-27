use std::time::SystemTime;
use database::schema::merchants;
use diesel::PgConnection;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Merchant {
    pub id:             String,
    pub merchant_name:  Option<String>,
    pub shortname:      Option<String>,
    pub created_at:     SystemTime,
    pub updated_at:     Option<SystemTime>,
    pub created_by:     Option<String>
}

#[derive(Insertable)]
#[table_name="merchants"]
pub struct Merchant_ {
    pub id:             String,
    pub merchant_name:  String,
    pub shortname:      String,
    pub created_by:     String
}

pub fn insert_into_merchant(new_merchant: Merchant_, connection: &PgConnection) -> Result<String, String> {
    match diesel::insert_into(merchants::table)
        .values(&new_merchant)
        .execute(connection) {
            Ok(_) => {
                return Ok(format!("insert new merchant successful"));
            },
            Err(err) => {
                return Err(format!("Error: {}", err));
        }
    }
}

pub fn get_all_merchants(connection: &PgConnection) -> Result<Vec<Merchant>, String> {
    use database::schema::merchants::dsl::merchants;
    match merchants.load::<Merchant>(connection) {
        Ok(merchant_) => return Ok(merchant_),
        Err(err) => return Err(format!("Error: {}", err)),
    }
}

pub fn get_merchant_by_created_by(created_by_: String, connection: &PgConnection) -> Result<Vec<Merchant>, String> {
    use database::schema::merchants::dsl::{merchants, created_by};
    match merchants.filter(created_by.eq(created_by_))
        .get_results(connection) {
            Ok(merchants_) => return Ok(merchants_),
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn get_merchant_by_name(name_: String, connection: &PgConnection) -> Result<Vec<Merchant>, String> {
    use database::schema::merchants::dsl::{merchants, merchant_name};
    match merchants.filter(merchant_name.eq(name_))
        .get_results(connection) {
            Ok(merchants_) => return Ok(merchants_),
            Err(err) => return Err(format!("Error: {}", err))
    }
}
