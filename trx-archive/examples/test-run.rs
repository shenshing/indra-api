use trx_archive::operation::{
    TrxArchive_,
    insert_new_trx,
    get_all_trxarchive,
    get_trxarchive_by_hash,
    get_trxarchive_by_sender_or_destination
};  
use database::db_connection::establish_connection;
use bigdecimal::BigDecimal;
use std::str::FromStr;

fn main() {
    let new_trx = TrxArchive_ {
        id:             String::from("id3"),
        block:          BigDecimal::from_str(&"0.10").unwrap(), 
        hash:           String::from("Hash"),
        sender:         String::from("sender1`"),
        destination:    String::from("destination"),
        amount:         BigDecimal::from_str(&"0.10").unwrap(),         
        fee:            BigDecimal::from_str(&"0.10").unwrap(), 
        memo:           String::from("memo"),
        created_by:     String::from("admin")
    };

    // let result = insert_new_trx(new_trx, &establish_connection());
    // println!("{:#?}", result);

    // let result = get_all_trxarchive(&establish_connection());
    // println!("{:#?}", result);
    
    // let result = get_trxarchive_by_hash(String::from("Hash"), &establish_connection());
    // println!("{:#?}", result);

    let result = get_trxarchive_by_sender_or_destination(String::from("sender1`1"), String::from("destination1"), &establish_connection());
    println!("{:#?}", result);
}