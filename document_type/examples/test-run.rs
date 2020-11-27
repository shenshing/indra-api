use document_type::operation::{get_document_by_name, get_all_document_type};
use database::db_connection::establish_connection;

fn main() {
    // let result = get_all_document_type(&establish_connection());
    // println!("{:#?}", result);

    let result = get_document_by_name(String::from("Passport"), &establish_connection());
    println!("{:#?}", result);
}