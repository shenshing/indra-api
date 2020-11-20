use actix_web::{web, post, get, Responder, HttpResponse};

use crate::users::{Register, insert_new_register};
use database::db_connection::establish_connection;

#[post("/registerbyemail")]
async fn register_by_email(data: web::Json<Register>) -> impl Responder {
    insert_new_register(data.into_inner(), &establish_connection());
    HttpResponse::Ok().body(String::from("everything ok"))
}

#[post("/registerbyphone")]
async fn register_by_phone(data: web::Json<Register>) -> impl Responder {
    insert_new_register(data.into_inner(), &establish_connection());
    HttpResponse::Ok().body(String::from("everything ok"))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// use actix_web::http::StatusCode;
#[get("/error")]
async fn error() -> impl Responder {
    // StatusCode::OK
    HttpResponse::Ok().body(String::from("everything ok"))
}