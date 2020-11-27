use std::time::SystemTime;
use bigdecimal::BigDecimal;
use database::schema::receipts;
use diesel::PgConnection;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Receipt {
    pub id:     String,
    pub receipt_no:     Option<String>,
    pub amount:         Option<BigDecimal>,
    pub location:       Option<String>,
    pub image_uri:      Option<String>,
    pub rewards:        Option<BigDecimal>,
    pub remark:         Option<String>,
    pub status:         Option<String>,
    pub created_at:     SystemTime,
    pub created_by:     Option<String>,
    pub updated_by:     Option<String>
}

// ID, 
// RECEIPT_NO, 
// AMOUNT, 
// LOCATION, 
// REWARDS, 
// STATUS, 
// CREATED_BY
#[derive(Insertable)]
#[table_name="receipts"]
pub struct Receipt_ {
    pub id:         String,
    pub receipt_no: String,
    pub amount:     BigDecimal,
    pub location:   String,
    pub rewards:    BigDecimal,
    pub status:     String,
    pub created_by: String
}

pub fn insert_new_receipt(rec_: Receipt_, connection: &PgConnection) -> Result<String, String> {
    match diesel::insert_into(receipts::table)
        .values(rec_)
        .execute(connection) {
            Ok(_) => {
                return Ok(format!("insert new receipt successful"));
            },
            Err(err) => {
                return Err(format!("error saving new receipt {}", err)); 
        }
    }
}

pub fn get_all_receipts(connection: &PgConnection) -> Result<Vec<Receipt>, String> {
    use database::schema::receipts::dsl::receipts;
    match receipts.load::<Receipt>(connection) {
        Ok(rec_) => return Ok(rec_),
        Err(err) => return Err(format!("Error: {}", err)),
    }
}

pub fn update_status_and_updated_by_by_id(id_: String, updated_by_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::receipts::dsl::{receipts, id, updated_by, status};
    match receipts.filter(id.eq(id_.clone()))
        .get_result::<Receipt>(connection) {
            Ok(_) => {
            match diesel::update(receipts)
                .filter(id.eq(id_.clone()))
                .set((
                    status.eq(String::from("Completed")),
                    updated_by.eq(updated_by_),
                ))
                .execute(connection) {
                    Ok(_) => return Ok(String::from("Successful update status id")),
                    Err(err) => return Err(format!("Error: {}", err))
                }
            },
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn get_receipts_by_updated_by_desc(updated_by_: String, connection: &PgConnection) -> Result<Vec<Receipt>, String> {
    use database::schema::receipts::dsl::{receipts, updated_by};
    match receipts.filter(updated_by.eq(updated_by_))
        .order_by(updated_by.desc())
        .get_results::<Receipt>(connection) {
            Ok(rec_) => return Ok(rec_),
            Err(err) => return Err(format!("Error: {}", err))
        }
}