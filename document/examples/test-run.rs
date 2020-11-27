use document::operation::{insert_into_documents, get_all_documents, Document_};
use database::db_connection::establish_connection;

fn main() {
    let new_docx = Document_ {
        id:                 String::from("id1"),
        documents_no:       String::from("001"),
        documenttype_id:    String::from("docx id001"),
        document_uri:       String::from("https:something.docx"),
        face_uri:           String::from("face uri"),
        issue_date:         String::from("Wed 25-11-2020"),
        expire_date:        String::from("Thu 26-11-2020"),
        created_by:         String::from("admin")
    };

    let result = insert_into_documents(new_docx, &establish_connection());
    println!("{:#?}", result);

    let result = get_all_documents(&establish_connection());
    println!("{:#?}", result);
}