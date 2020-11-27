// use branch::operation::{get_all_branch, insert_new_branch, Branch_};
use database::db_connection::establish_connection;
use branch::operation::{Branch_, Branches_};
use bigdecimal::BigDecimal;
use std::str::FromStr; 
// use branch::operation::get_branch_by_branch_name;
// use branch::operation::get_all_branch_list;
// use branch::operation::get_branch_by_granted;
// use branch::operation::update_branch_by_id;
use branch::operation::update_granted_by_name;

fn main() {
    // let new_branch = Branch_ {
    //     id:             String::from("id2"),
    //     merchants_id:   String::from("0002"),
    //     branches_name:  String::from("DongKao"),
    //     address:        String::from("DongKao District"),
    //     reward_rates:   BigDecimal::from_str(&"0.10").unwrap(),
    //     asset_code:     String::from("SEL"),
    //     minimum_spend:  BigDecimal::from_str(&"0.5").unwrap(),
    //     approval_code:  String::from("auth"),
    //     logo_uri:       String::from("logo uri"),
    //     created_by:     String::from("branch admin")
    // };

    // let result = insert_new_branch(new_branch, &establish_connection());
    // println!("{:#?}", result);

    // let result = get_all_branch(&establish_connection());
    // println!("{:#?}", result);

    // let result = get_branch_by_branch_name(String::from("DaunPenh"), &establish_connection());
    // println!("{:#?}", result);

    // let result = get_branch_by_granted(String::from("user"), &establish_connection());
    // println!("{:#?}", result);

    // let new_data = Branches_ {
    //     address:    String::from("new address"),
    //     reward_rates:   BigDecimal::from_str(&"0.7").unwrap(),
    //     asset_code:     String::from("tSEL"),
    //     minimum_spend:  BigDecimal::from_str(&"0.3").unwrap(),
    //     approval_code:  String::from("auth2"),
    //     logo_uri:       String::from("new verss"),
    //     is_active:      false
    // };

    // let result = update_branch_by_id(String::from("id2"), new_data, &establish_connection());
    // println!("{:#?}", result);

    let result = update_granted_by_name(String::from("DaunP1enh"), String::from("admin"), &establish_connection());
    println!("{:#?}", result);
}