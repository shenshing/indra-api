use hashval::operation::{Hashval_, insert_new_hashval, get_all_hashval, update_valid_and_hashval_by_id};
use database::db_connection::establish_connection;

fn main() {
    // let result = insert_new_hashval(
    //     Hashval_ {
    //         id:     String::from("id1"),
    //         hashs:  String::from("hash value"),
    //         created_by: String::from("admin")
    //     },
    //     &establish_connection()
    // );

    // println!("{:#?}", result);

    // let result = get_all_hashval(&establish_connection());
    // println!("{:#?}", result);

    let result = update_valid_and_hashval_by_id(String::from("id1"), false, String::from("admin1"), &establish_connection());
    println!("{:#?}", result);
}