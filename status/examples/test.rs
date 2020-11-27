use status::operation::get_status_by_name;
use database::db_connection::establish_connection;

fn main() {
    let result = get_status_by_name(String::from("inactive"), &establish_connection());
    println!("{:#?}", result);
}