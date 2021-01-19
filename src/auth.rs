// Defines the Auth endpoint
use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Responder, Scope};
use std::env;

#[get("/")]
async fn main_entry() -> impl Responder {
    format!("Hello Stranger from /")
}

#[get("/h")]
async fn main_entry_2() -> impl Responder {
    format!("Hello Stranger from /h")
}

#[get("/e")]
async fn main_entry_3() -> impl Responder {
    format!("Hello Stranger from /e")
}
pub fn get_routes(mount_path: &str) -> Scope {
    web::scope(&mount_path)
        .service(main_entry)
        .service(main_entry_2)
        .service(main_entry_3)
}
