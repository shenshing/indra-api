use database::users::Users;
use database::users;
use database::db_connection::establish_connection;
use crate::users::insert_register_by_email;


fn main() {
    // println!("Hello World");
    // let now = default_timestamp();
    // let user = Users {
    //     id:             String::from("000000001"),
    //     first_name:     Some(String::from("Shing")),
    //     mid_name:       Some(String::from("Shen")),
    //     last_name:      Some(String::from("LY")),
    //     description:    Some(String::from("this is description")),
    //     email:          Some(String::from("lyshenshing20@gmail.com")),
    //     gender:         Some(String::from("M")),
    //     profile_img:    Some(String::from("profile_img")),
    //     wallet:         Some(String::from("wallet")),
    //     seed:           Some(String::from("seed")),
    //     password:       String::from("password"),
    //     temp_token:     Some(String::from("temp token")),
    //     pin:            Some(String::from("pin")),
    //     user_status:    Some(String::from("user status")),
    //     is_partner:     true,
    //     nationality:    Some(String::from("nationality")),
    //     occupation:     Some(String::from("occupation")),
    //     phonenumber:    Some(String::from("phonenumber")),
    //     documents_id:   Some(String::from("documents id")),
    //     status_id:      Some(String::from("status id")),
    //     address:        Some(String::from("address")),
    //     created_at:     now,
    //     updated_at:     Some(now),
    //     created_by:     Some(String::from("admin")),
    //     updated_by:     Some(String::from("admin"))
    // };

    // users::insert_new_user(&establish_connection(), user);

    insert_register_by_email();
}