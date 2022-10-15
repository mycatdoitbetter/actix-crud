use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    match PgConnection::establish(&database_url) {
        Ok(conn) => conn,
        Err(e) => panic!("Error connecting to {}: {}", database_url, e),
    }
}
