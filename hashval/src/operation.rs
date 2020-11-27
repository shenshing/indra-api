use std::time::SystemTime;
use database::schema::hashval;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable, Debug)]
pub struct Hashval {
    pub id:         String,
    pub hashs:      Option<String>,
    pub is_valid:   bool,
    pub created_at: SystemTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>
}

#[derive(Insertable)]
#[table_name="hashval"]
pub struct Hashval_ {
    pub id:         String,
    pub hashs:      String,
    pub created_by: String
}

pub fn insert_new_hashval(new_hashval: Hashval_, connection: &PgConnection) -> Result<String, String> {
    match diesel::insert_into(hashval::table)
        .values(&new_hashval)
        .execute(connection) {
            Ok(_) => {
                return Ok(format!("insert new hashval successful"));
            },
            Err(err) => {
                return Err(format!("Error: {}", err));
        }
    }
}

pub fn get_all_hashval(connection: &PgConnection) -> Result<Vec<Hashval>, String> {
    use database::schema::hashval::dsl::hashval;
    match hashval.load::<Hashval>(connection) {
        Ok(hashval_) => return Ok(hashval_),
        Err(err) => return Err(format!("Error: {}", err)),
    }
}

pub fn update_valid_and_hashval_by_id(id_: String, valid_: bool, updated_by_: String, connection: &PgConnection) -> Result<String, String> {
    use database::schema::hashval::dsl::{hashval, id, is_valid, updated_by};
    match hashval.filter(id.eq(id_.clone()))
        .get_result::<Hashval>(connection) {
            Ok(_) => {
            match diesel::update(hashval)
                .filter(id.eq(id_.clone()))
                .set((
                    is_valid.eq(valid_),
                    updated_by.eq(updated_by_),
                ))
                .execute(connection) {
                    Ok(_) => return Ok(String::from("Successful updating")),
                    Err(err) => return Err(format!("Error: {}", err))
                }
            },
            Err(err) => return Err(format!("Error: {}", err))
    }
}