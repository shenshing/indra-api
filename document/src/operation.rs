use std::time::SystemTime;
use database::schema::documents;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Document {
    pub id:                 String,
    pub documents_no:       Option<String>,
    pub documenttype_id:    Option<String>,
    pub document_uri:       String,
    pub face_uri:           String,
    pub issue_date:         String,
    pub expire_date:        String,
    pub created_at:         SystemTime,
    pub updated_at:         Option<SystemTime>,
    pub created_by:         Option<String>,
    pub updated_by:         Option<String>
}

#[derive(Insertable)]
#[table_name="documents"]
pub struct Document_ {
    pub id:                 String,
    pub documents_no:       String,
    pub documenttype_id:    String,
    pub document_uri:       String,
    pub face_uri:           String,
    pub issue_date:         String,
    pub expire_date:        String,
    pub created_by:         String,
}

pub fn insert_into_documents(new_docx: Document_, connection: &PgConnection) -> Result<String, String> {
    match diesel::insert_into(documents::table)
        .values(&new_docx)
        .execute(connection) {
            Ok(_) => {
                return Ok(format!("insert new document successful"));
            },
            Err(err) => {
                return Err(format!("error saving new document {}", err));
        }
    }
}

pub fn get_all_documents(connection: &PgConnection) -> Result<Vec<Document>, String> {
    use database::schema::documents::dsl::*;
    match documents.load::<Document>(connection) {
        Ok(docx) => return Ok(docx),
        Err(err) => return Err(format!("Error: {}", err)),
    }
}