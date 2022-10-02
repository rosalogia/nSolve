use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::Config;

pub fn establish_connection(config: &Config) -> PgConnection {
    let database_url = &config.database_url;
    PgConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

