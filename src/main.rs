use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;

mod models;
mod postgres;
mod controller;
mod schema;
mod services;

use crate::controller::{create, read, update, delete};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match dotenv() {
        Ok(_) => {}
        Err(e) => panic!("Error loading .env file: {}", e),
    }

    env::set_var("RUST_LOG", "actix_web=debug");

    let host = env::var("HOST").expect("HOST environment variable must be set");
    let port = env::var("PORT")
        .expect("PORT environment variable must be set")
        .parse::<u16>().unwrap();

    let server = HttpServer::new(|| {
        App::new()
            .service(create)
            .service(read)
            .service(update)
            .service(delete)
    });

    println!("Server started! 🦀");
    println!("Running at {}:{}", host, port);

    server.bind((host, port))?.run().await
}
