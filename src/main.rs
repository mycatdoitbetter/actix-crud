mod configs;
mod modules;

use std::env;
use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use lazy_static::lazy_static;

use modules::posts;

lazy_static! {
    static ref INITIATED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}
struct Environment {
    pub port: u16,
    pub host: String,
}

impl Environment {
    fn new() -> Self {
        dotenv().expect("Failed to load .env file");

        env::set_var("RUST_LOG", "actix_web=debug");

        let port = env::var("PORT")
            .expect("PORT is not set.")
            .parse::<u16>()
            .expect("PORT is not a valid number.");

        let host = env::var("HOST").expect("HOST is not set.");

        configs::postgres::establish_connection();

        Environment { port, host }
    }
}

fn new_environment_before_initialize_application() -> Option<Environment> {
    let mut initiated = INITIATED.lock().unwrap();

    if *initiated == false {
        let environment = Environment::new();

        *initiated = true;

        Some(environment)
    } else {
        None
    }
}

fn config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/posts").configure(posts::configuration));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = new_environment_before_initialize_application().expect("Environment is already initiated.");

    let server = HttpServer::new(move || App::new().configure(config));

    println!("Server started! 🦀");
    println!("Running at {}:{}", env.host, env.port);

    server.bind((env.host, env.port))?.run().await
}
