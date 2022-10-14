use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    match PgConnection::establish(&database_url) {
        Ok(conn) => conn,
        Err(e) => panic!("Error connecting to {}: {}", database_url, e),
    }
}
