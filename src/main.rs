use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Responder};
use config::read_config;
use std::env;
mod auth;
mod config;
#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[get("/")]
async fn main_entry() -> impl Responder {
    format!("Hello Stranger")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("Path is {:?}", path);
    let read_config = read_config().expect("to read config");
    println!("read info is {:?}", read_config);
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(main_entry)
            .service(fs::Files::new("/static", "./public").show_files_listing())
            .service(auth::get_routes("/users"))
    })
    .bind("0.0.0.0:7003")?
    .run()
    .await
}
