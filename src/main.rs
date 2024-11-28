use actix_web::{web, App, HttpServer};
mod db;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = web::Data::new(db::connect().await);

    HttpServer::new(move || App::new().configure(handlers::config(pool.clone())))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
