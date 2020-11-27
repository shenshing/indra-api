use api_acc::api_account::{insert_api_key, get_acc_by_apikey};
use api_acc::api_account::{ApiAccount, Api};
use database::db_connection::establish_connection;

fn main() {
    // let acc = Api {
    //     id: String::from("id1"),
    //     apikey: String::from("apikey"),
    //     apisec: String::from("apiacc"),
    // };

    // let result = insert_api_key(acc, &establish_connection());
    // println!("{:#?}", result);


    let result = get_acc_by_apikey(String::from("apikey"), &establish_connection());
    println!("{:#?}", result);
}