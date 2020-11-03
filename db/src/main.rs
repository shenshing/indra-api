use subwallet::crypto::Ecdsa;
use subwallet::wallet::Address;

mod storage;

fn main() {
    //write to db (location: /home/your-user-name/.sel_db/xt-selendra-db)
    let label = String::from("user_c");
    let address = Address::generate::<Ecdsa>();
    storage::save_to_db(address, label.clone());

    //read from db (location: /home/your-user-name/.sel_db/xt-selendra-db)
    let address_return = storage::read_by_label(label.clone());
    println!("{:#?}", address_return);
}

