use std::time::SystemTime;
use diesel::PgConnection;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct DocumentType {
    pub id:             String,
    pub document_name:  Option<String>,
    pub created_at:     SystemTime,
    pub updated_by:     Option<SystemTime>
}

pub fn get_document_by_name(name_: String, connection: &PgConnection) -> Result<Vec<DocumentType>, String> {
    use database::schema::documenttype::dsl::{documenttype, document_name};
    match documenttype.filter(document_name.eq(name_))
        .get_results(connection) {
            Ok(docx_type) => return Ok(docx_type),
            Err(err) => return Err(format!("Error: {}", err))
        }
}

pub fn get_all_document_type(connection: &PgConnection) -> Result<Vec<DocumentType>, String> {
    use database::schema::documenttype::dsl::documenttype;
    match documenttype.load::<DocumentType>(connection) {
        Ok(docx_type) => return Ok(docx_type),
        Err(err) => return Err(format!("Error: {}", err)),
    }
}