use std::time::SystemTime;
use database::schema::branches;
use diesel::PgConnection;
use diesel::prelude::*;

use bigdecimal::BigDecimal;
#[derive(Queryable, Debug)]
pub struct Branch {
    id:             String, 
    merchants_id:   Option<String>,
    branches_name:  Option<String>,
    address:        Option<String>,
    reward_rates:   Option<BigDecimal>,
    asset_code:     Option<String>,
    minimum_spend:  Option<BigDecimal>,
    approval_code:  Option<String>,
    logo_uri:       Option<String>,
    is_active:      bool,
    created_at:     SystemTime,
    updated_at:     Option<SystemTime>,
    created_by:     Option<String>,
    granted_for:    Option<String>
}

#[derive(Queryable, Debug)]
pub struct Branches {
    branches_name:  Option<String>, 
    address:        Option<String>,
    reward_rates:   Option<BigDecimal>,
    asset_code:     Option<String>,
    minimum_spend:  Option<BigDecimal>,
    logo_uri:       Option<String>,
    is_active:      bool
}

#[derive(Insertable)]
#[table_name="branches"]
pub struct Branch_ {
    pub id:             String, 
    pub merchants_id:   String,
    pub branches_name:  String,
    pub address:        String,
    pub reward_rates:   BigDecimal,
    pub asset_code:     String,
    pub minimum_spend:  BigDecimal,
    pub approval_code:  String,
    pub logo_uri:       String,
    pub created_by:     String,
}

pub struct Branches_ {
    pub address:        String,
    pub reward_rates:   BigDecimal,
    pub asset_code:     String,
    pub minimum_spend:  BigDecimal,
    pub approval_code:  String,
    pub logo_uri:       String,
    pub is_active:      bool
}

pub fn insert_new_branch(branch_: Branch_, connection: &PgConnection) -> Result<String, String> {
    match diesel::insert_into(branches::table)
        .values(branch_)
        .execute(connection) {
            Ok(_) => {
                return Ok(format!("insert new branch successful"));
            },
            Err(err) => {
                return Err(format!("error saving new branch {}", err)); 
        }
    }
}

pub fn get_all_branch(connection: &PgConnection) -> Result<Vec<Branch>, String> {
    use database::schema::branches::dsl::*;
    match branches.load::<Branch>(connection) {
        Ok(branch) => return Ok(branch),
        Err(err)   => return Err(format!("Error: {}", err)),
    }
}

pub fn get_branch_by_branch_name(branch_name_: String, connection: &PgConnection) -> Result<Branch, String> {    
    use database::schema::branches::dsl::{branches, branches_name};
    match branches.filter(branches_name.eq(branch_name_))
        .get_result(connection) {
            Ok(branch) => return Ok(branch),
            Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn get_all_branch_list(created_by_: String, connection: &PgConnection) -> Result<Vec<Branches>, String> {
    use database::schema::branches::dsl::*;
    match branches
    .filter(created_by.eq(created_by_))
    .select(
        (branches_name, address, reward_rates, asset_code, minimum_spend, logo_uri, is_active)
    ).load::<Branches>(connection) {
        Ok(branches_) =>  return Ok(branches_),
        Err(err) => return Err(format!("{}", err)),
    }
}

pub fn get_branch_by_granted(granted_for_: String, connection: &PgConnection) -> Result<Vec<Branches>, String> {
    use database::schema::branches::dsl::*;
    match branches
    .filter(granted_for.eq(granted_for_))
    .select(
        (branches_name, address, reward_rates, asset_code, minimum_spend, logo_uri, is_active)
    ).load::<Branches>(connection) {
        Ok(branches_) =>  return Ok(branches_),
        Err(err) => return Err(format!("{}", err)),
    }
}

// ADDRESS         = :ADDRESS,
// REWARD_RATES    = :REWARD_RATES,
// ASSET_CODE      = :ASSET_CODE,
// MINIMUM_SPEND   = :MINIMUM_SPEND,
// APPROVAL_CODE   = :APPROVAL_CODE,
// LOGO_URI        = :LOGO_URI,
// IS_ACTIVE       = :IS_ACTIVE


pub fn update_branch_by_id(id_: String, new_data: Branches_,  connection: &PgConnection) ->  Result<String, String> {
    use database::schema::branches::dsl::*;
    match branches.filter(id.eq(id_.clone()))
        .get_result::<Branch>(connection) {
            Ok(_) => {
                match diesel::update(branches)
                    .filter(id.eq(id_.clone()))
                    .set((
                        address.eq(new_data.address),
                        reward_rates.eq(new_data.reward_rates),
                        asset_code.eq(new_data.asset_code),
                        minimum_spend.eq(new_data.minimum_spend),
                        approval_code.eq(new_data.approval_code),
                        logo_uri.eq(new_data.logo_uri),
                        is_active.eq(new_data.is_active),
                    ))
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update branch")),
                        Err(err) => return Err(format!("Error: {}", err)) 
                    }
            },
            Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn update_granted_by_name(name_: String, granted_for_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::branches::dsl::{branches, branches_name, granted_for};
    match branches.filter(branches_name.eq(name_.clone()))
        .get_result::<Branch>(connection) {
            Ok(_) => {
                match diesel::update(branches)
                    .filter(branches_name.eq(name_.clone()))
                    .set(
                        granted_for.eq(granted_for_)
                    )
                    .execute(connection) {
                        Ok(_) => return Ok(String::from("Successful update granted info")),
                        Err(err) => return Err(format!("Error: {}", err)) 
                    }
            },
            Err(err) => return Err(format!("Error: {}", err))
        }
}