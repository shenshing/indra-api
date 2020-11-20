use std::time::SystemTime;

use diesel::prelude::*;
use diesel::PgConnection;
use database::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Debug)]
#[table_name="users"]
pub struct Users {
    pub id:             String,
    pub first_name:     Option<String>,
    pub mid_name:       Option<String>,
    pub last_name:      Option<String>,
    pub description:    Option<String>,
    pub email:          Option<String>,
    pub gender:         Option<String>,
    pub profile_img:    Option<String>,
    pub wallet:         Option<String>,
    pub seed:           Option<String>,
    pub password:       String,
    pub temp_token:     Option<String>,
    pub pin:            Option<String>,
    pub user_status:    Option<String>,
    pub is_partner:     bool,
    pub nationality:    Option<String>,
    pub occupation:     Option<String>,
    pub phonenumber:    Option<String>,
    pub documents_id:   Option<String>,
    pub status_id:      Option<String>,
    pub address:        Option<String>,
    pub created_at:     SystemTime,
    pub updated_at:     Option<SystemTime>,
    pub created_by:     Option<String>,
    pub updated_by:     Option<String>
}

pub struct NewUser {
    pub id:             String,
    pub first_name:     Option<String>,
    pub mid_name:       Option<String>,
    pub last_name:      Option<String>,
    pub description:    Option<String>,
    pub email:          Option<String>,
    pub gender:         Option<String>,
    pub profile_img:    Option<String>,
    pub wallet:         Option<String>,
    pub seed:           Option<String>,
    pub password:       String,
    pub temp_token:     Option<String>,
    pub pin:            Option<String>,
    pub user_status:    Option<String>,
    pub is_partner:     bool,
    pub nationality:    Option<String>,
    pub occupation:     Option<String>,
    pub phonenumber:    Option<String>,
    pub documents_id:   Option<String>,
    pub status_id:      Option<String>,
    pub address:        Option<String>,
    pub created_at:     SystemTime,
    pub updated_at:     Option<SystemTime>,
    pub created_by:     Option<String>,
    pub updated_by:     Option<String>
}

pub fn insert_new_user(connection: &PgConnection, new_user: Users) {
    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection) {
            Ok(_) => {
                println!("insert new account successful");
            },
            Err(err) => {
                println!("error saving new account {}", err);
        }
    }
}


#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name="users"]
pub struct Register {
    pub id:             String,
    pub phonenumber:    String,
    pub email:          String,
    pub password:       String,
    pub temp_token:     String,
    pub status_id:      String,
}
pub fn insert_new_register(new_register: Register, connection: &PgConnection) {
    match diesel::insert_into(users::table)
        .values(&new_register)
        .execute(connection) {
            Ok(_) => {
                println!("insert new account successful");
            },
            Err(err) => {
                println!("error saving new account {}", err);
        }
    }
}

#[derive(Insertable)]
#[table_name="users"]
pub struct SetupWallet {
    pub id:         String,
    pub password:   String,
    pub wallet:     String,
    pub seed:       String,
    pub created_by: String     
}

pub fn insert_for_setup_wallet(new_wallet: SetupWallet, connection: &PgConnection) {
    match diesel::insert_into(users::table)
        .values(&new_wallet)
        .execute(connection) {
            Ok(_) => {
                println!("insert new wallet successful");
            },
            Err(err) => {
                println!("error saving new wallet {}", err);
            }
    }
}

pub fn get_all_users(connection: &PgConnection) -> Vec<Users> {
    use database::schema::users::dsl::*;

    let user_list = users.load::<Users>(connection)
        .expect("Error retrieve user from database");
    return user_list;
}

pub fn get_user_by_email(email_: String, connection: &PgConnection) -> Result<Users, String> {
    use database::schema::users::dsl::{users, email};
    match users.filter(email.like(email_))
        .get_result(connection) {
            Ok(user) => return Ok(user),
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn get_user_by_id(id_: String, connection: &PgConnection) -> Result<Users, String> {
    use database::schema::users::dsl::{users, id};
    match users.filter(id.eq(id_))
        .get_result(connection) {
            Ok(user) => return Ok(user),
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn get_user_by_phonenumber(phonenumber_: String, connection: &PgConnection) -> Result<Users, String> {
    use database::schema::users::dsl::{users, phonenumber};
    match users.filter(phonenumber.eq(phonenumber_))
        .get_result(connection) {
            Ok(user) => return Ok(user),
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn get_seed_by_id(id_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, id};
    match users.filter(id.eq(id_))
        .get_result::<Users>(connection) {
            Ok(user) => {
                match user.seed {
                    Some(seed) => return Ok(seed),
                    None => Err(format!("Seed is None"))
                }
            },
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn get_pin_by_id(id_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, id};
    match users.filter(id.eq(id_))
        .get_result::<Users>(connection) {
            Ok(user) => {
                match user.pin {
                    Some(pin) => return Ok(pin),
                    None => Err(format!("Pin is None"))
                }
            },
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn get_wallet_by_phone(phone_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, phonenumber};
    match users.filter(phonenumber.eq(phone_))
        .get_result::<Users>(connection) {
            Ok(user) => {
                match user.wallet {
                    Some(wallet) => return Ok(wallet),
                    None => Err(format!("Wallet is None"))
                }
            },
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn get_user_token_by_id(id_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, id};
    match users.filter(id.eq(id_))
        .get_result::<Users>(connection) {
            Ok(user) => {
                match user.temp_token {
                    Some(temp_token) => return Ok(temp_token),
                    None => Err(format!("Token is None"))
                }
            },
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn get_user_token_by_phonenumber(phone_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, phonenumber};
    match users.filter(phonenumber.eq(phone_))
        .get_result::<Users>(connection) {
            Ok(user) => {
                match user.temp_token {
                    Some(temp_token) => return Ok(temp_token),
                    None => Err(format!("Token is None"))
                }
            },
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn update_token_and_status_by_id(id_: String, token_: String, status_id_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, id, status_id, temp_token};
    match users.filter(id.eq(id_.clone()))
        .get_result::<Users>(connection) {
            Ok(_) => {
            match diesel::update(users)
                .filter(id.eq(id_.clone()))
                .set((
                    temp_token.eq(token_),
                    status_id.eq(status_id_),
                ))
                .execute(connection) {
                    Ok(_) => return Ok(String::from("Successful update token and staus id")),
                    Err(err) => return Err(format!("Error: {}", err))
                }
            },
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn update_token_and_status_by_phone(phone_: String, token_: String, status_id_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, phonenumber, status_id, temp_token};
    match users.filter(phonenumber.eq(phone_.clone()))
        .get_result::<Users>(connection) {
            // .execute(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(phonenumber.eq(phone_.clone()))
                    .set((
                        temp_token.eq(token_),
                        status_id.eq(status_id_),
                    ))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update token and staus id")),
                        Err(err) => return Err(format!("Error: {}", err))
                }
            },
            Err(err) => return Err(format!("Error: {}", err))
    }
}
//88888888
pub fn update_token_and_status_by_email(email_: String, token_: String, status_id_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, email, status_id, temp_token};
    match users.filter(email.eq(email_.clone()))
        .get_result::<Users>(connection) {
            // .execute(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(email.eq(email_.clone()))
                    .set((
                        temp_token.eq(token_),
                        status_id.eq(status_id_),
                    ))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update token and staus id")),
                        Err(err) => return Err(format!("Error: {}", err))
                }
            },
            Err(err) => return Err(format!("Error: {}", err))
    }
}

//888888888888888888888
pub fn update_phone_and_token_by_id(id_: String, phone_: String, token_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, id, phonenumber, temp_token};
    match users.filter(id.eq(id_.clone()))
        .get_result::<Users>(connection) {
            // .execute(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(id.eq(id_.clone()))
                    .set((
                        phonenumber.eq(phone_),
                        temp_token.eq(token_),
                    ))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update phone number and token")),
                        Err(err) => return Err(format!("Error: {}", err))
                }
            },
            Err(err) => return Err(format!("Error: {}", err))
    }
}

pub fn update_status_by_id(id_: String, status_id_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, status_id, id};
    match users.filter(id.eq(id_.clone()))
        .get_result::<Users>(connection) {
        // .execute(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(id.eq(id_.clone()))
                    .set(status_id.eq(status_id_))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update status id")),
                        Err(err) => return Err(format!("Error: {}", err))
                    }
            },
            Err(err) => return Err(format!("Error: {}", err))
        }
}

//set up user profile
pub fn update_user_profile_by_id(
    id_:    String,
    first_: String,
    mid_:   String,
    last_:  String,
    gender_: String,
    status_: String,
    connection: &PgConnection
) -> Result<String, String> {
    use database::schema::users::dsl::{users, id, first_name, mid_name, last_name, gender, status_id};
    match users.filter(id.eq(id_.clone()))
        // .execute(connection) {
            .get_result::<Users>(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(id.eq(id_.clone()))
                    .set((
                        first_name.eq(first_),
                        mid_name.eq(mid_),
                        last_name.eq(last_),
                        gender.eq(gender_),
                        status_id.eq(status_)
                    ))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update user info")),
                        Err(err) => return Err(format!("Error: {}", err))
                    }
            },
            Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn update_wallet_by_id(id_: String, wallet_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, id, wallet};
    match users.filter(id.eq(id_.clone()))
        .get_result::<Users>(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(id.eq(id_.clone()))
                    .set(wallet.eq(wallet_))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update user wallet")),
                        Err(err) => return Err(format!("Error: {}", err))
                    }
            },
                Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn update_partner_to_true(id_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, id, is_partner};
    match users.filter(id.eq(id_.clone()))
        .get_result::<Users>(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(id.eq(id_.clone()))
                    .set(is_partner.eq(true))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update Partnership")),
                        Err(err) => return Err(format!("Error: {}", err))
                    }
            },
                Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn update_password_by_phonenumber(phone_: String, password_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, password, phonenumber};
    match users.filter(phonenumber.eq(phone_.clone()))
        .get_result::<Users>(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(phonenumber.eq(phone_.clone()))
                    .set(password.eq(password_))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update Password")),
                        Err(err) => return Err(format!("Error: {}", err))
                    }
            },
                Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn update_password_by_email(email_: String, password_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, password, email};
    match users.filter(email.eq(email_.clone()))
        .get_result::<Users>(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(email.eq(email_.clone()))
                    .set(password.eq(password_))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update Password")),
                        Err(err) => return Err(format!("Error: {}", err))
                    }
            },
                Err(err) => return Err(format!("Error: {}", err))
        }
}
//88888888
pub fn update_password_by_id(id_: String, password_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, password, id};
    match users.filter(id.eq(id_.clone()))
        .get_result::<Users>(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(id.eq(id_.clone()))
                    .set(password.eq(password_))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update Password")),
                        Err(err) => return Err(format!("Error: {}", err))
                    }
            },
                Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn update_pin_by_id(id_: String, pin_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, pin, id};
    match users.filter(id.eq(id_.clone()))
        .get_result::<Users>(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(id.eq(id_.clone()))
                    .set(pin.eq(pin_))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update PIN")),
                        Err(err) => return Err(format!("Error: {}", err))
                    }
            },
                Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn update_token_by_phonenumber(phone_: String, token_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, temp_token, phonenumber};
    match users.filter(phonenumber.eq(phone_.clone()))
        .get_result::<Users>(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(phonenumber.eq(phone_.clone()))
                    .set(temp_token.eq(token_))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update Password")),
                        Err(err) => return Err(format!("Error: {}", err))
                    }
            },
                Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn update_document_by_id(id_: String, document_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, documents_id, id};
    match users.filter(id.eq(id_.clone()))
        .get_result::<Users>(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(id.eq(id_.clone()))
                    .set(documents_id.eq(document_))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update Document ID")),
                        Err(err) => return Err(format!("Error: {}", err))
                    }
            },
                Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn update_phonenumber_by_id(id_: String, phone_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, phonenumber, id};
    match users.filter(id.eq(id_.clone()))
        .get_result::<Users>(connection) {
            Ok(_) => {
                match diesel::update(users)
                    .filter(id.eq(id_.clone()))
                    .set(phonenumber.eq(phone_))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update Phone Number")),
                        Err(err) => return Err(format!("Error: {}", err))
                    }
            },
                Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn delete_user_by_phonenumber(phone_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::users::dsl::{users, phonenumber};
    match diesel::delete(users.filter(phonenumber.eq(phone_)))
        // .execute(connection) {
        .get_result::<Users>(connection) {
            Ok(_) => return Ok(String::from("Successful delete user")),
            Err(err) => return Err(format!("Error: {}", err))
        }
}
// pub fn insert_register_by_email() {
//     let now = default_timestamp();
//     let new_user = Users {
//         id:           String::from("id1"),  
//         first_name:   None,
//         mid_name:     None,
//         last_name:    None,
//         description:  None,
//         email:        None,
//         gender:       None,
//         profile_img:  None,
//         wallet:       None,
//         seed:         None,
//         password:     String::from("123"),
//         temp_token:   None,
//         pin:          None,
//         user_status:  None,
//         is_partner:   false,
//         nationality:  None, 
//         occupation:   None,  
//         phonenumber:  None,
//         documents_id: None, 
//         status_id:    None,
//         address:      None,
//         created_at:   now,
//         updated_at:   None,
//         created_by:   None,
//         updated_by:   None,
//     };

//     insert_new_user(&establish_connection(), new_user);
// }