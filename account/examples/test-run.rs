// use account::users::insert_new_register;
// use account::users::insert_for_setup_wallet;
// use account::users::get_all_users;
use account::users::get_user_by_email;
// use account::users::get_user_by_id;
// use account::users::get_user_by_phonenumber;
// use account::users::get_seed_by_id;
// use account::users::get_pin_by_id;
// use account::users::get_wallet_by_phone;
// use account::users::get_user_token_by_id;
// use account::users::get_user_token_by_phonenumber;
// use account::users::update_token_and_status_by_id;
// use account::users::update_status_by_id;
// use account::users::update_user_profile_by_id;
// use account::users::update_wallet_by_id;
// use account::users::update_partner_to_true;
// use account::users::update_password_by_email;
// use account::users::delete_user_by_phonenumber;
use database::db_connection::establish_connection;



fn main() {
    // insert_new_register();
    // insert_for_setup_wallet(&establish_connection());
    // let result = get_all_users(&establish_connection());
    // println!("{:#?}", result);

    let result = get_user_by_email(String::from("lyshenshing20@gmail.com"), &establish_connection());
    println!("{:#?}", result);

    // let result = update_token_and_status_by_id(String::from("id33"), String::from("new token1"), String::from("new id1"), &establish_connection());
    // println!("{:#?}", result);

    // let result = update_status_by_id(String::from("0000000011"), String::from("new id99"), &establish_connection());
    // println!("{:#?}", result);

    // let result = update_user_profile_by_id(String::from("id12"), String::from("all old"), String::from("all old"), String::from("all old"), String::from("M"), String::from("all  old"), &establish_connection());
    // println!("{:#?}", result);

    // let result = update_wallet_by_id(String::from("id31"), String::from("new wallet_"), &establish_connection());
    // println!("{:#?}", result);

    // let result = update_password_by_email(String::from("lyshenshing20@gmail.com.kh"), String::from("singsecret"), &establish_connection());
    // println!("{:#?}", result);

    // let result = delete_user_by_phonenumber(String::from("0966508960"), &establish_connection());
    // println!("{:#?}", result);
}


// use account::endpoint::register_by_email;

// use actix_web::{HttpServer, App};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(register_by_email)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }