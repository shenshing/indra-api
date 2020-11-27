use std::time::SystemTime;
use bigdecimal::BigDecimal;
use database::schema::trxarchive;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable, Debug)]
pub struct TrxArchive {
    pub id:             String, 
    pub block:          Option<BigDecimal>,
    pub hash:           Option<String>,
    pub sender:         Option<String>,
    pub distination:    Option<String>,
    pub amount:         Option<BigDecimal>,
    pub fee:            Option<BigDecimal>,
    pub memo:           Option<String>,
    pub created_at:     SystemTime,
    pub created_by:     Option<String>,
    pub updated_by:     Option<String>
}

#[derive(Insertable, Debug)]
#[table_name="trxarchive"]
pub struct TrxArchive_ {
    pub id:             String,
    pub block:          BigDecimal,
    pub hash:           String,
    pub sender:         String,
    pub destination:    String,
    pub amount:         BigDecimal,
    pub fee:            BigDecimal,
    pub memo:           String,
    pub created_by:     String
}

pub fn insert_new_trx(trx_: TrxArchive_, connection: &PgConnection) -> Result<String, String> {
    match diesel::insert_into(trxarchive::table)
        .values(trx_)
        .execute(connection) {
            Ok(_) => {
                return Ok(format!("insert new transaction successful"));
            },
            Err(err) => {
                return Err(format!("error saving new transaction {}", err)); 
        }
    }
}

pub fn get_all_trxarchive(connection: &PgConnection) -> Result<Vec<TrxArchive>, String> {
    use database::schema::trxarchive::dsl::trxarchive;
    match trxarchive.load::<TrxArchive>(connection) {
        Ok(trx_) => return Ok(trx_),
        Err(err) => return Err(format!("Error: {}", err)),
    }
}

pub fn get_trxarchive_by_hash(hash_: String, connection: &PgConnection) -> Result<TrxArchive, String> {
    use database::schema::trxarchive::dsl::{trxarchive, hash};
    match trxarchive.filter(hash.eq(hash_))
        .get_result(connection) {
            Ok(trx_) => return Ok(trx_),
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn get_trxarchive_by_sender_or_destination(sender_: String, dest_: String, connection: &PgConnection) -> Result<Vec<TrxArchive>, String> {
    use database::schema::trxarchive::dsl::{trxarchive, sender, destination};
    match trxarchive.filter(sender.eq(sender_))
        .or_filter(destination.eq(dest_))
        .get_results(connection) {
            Ok(trx_) => return Ok(trx_),
            Err(err) => return Err(format!("Error: {}", err))
        }
}