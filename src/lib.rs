pub mod helpers;
pub mod models;
pub mod schema;

use std::env;

use diesel::{sqlite::SqliteConnection, Connection};
use dotenv::dotenv;

pub fn get_db() -> SqliteConnection {
    #[cfg(debug_assertions)]
    dotenv().ok();
    #[cfg(debug_assertions)]
    let env_url = env::var("DATABASE_URL").expect("Error while reading DB URL");
    #[cfg(debug_assertions)]
    let db_url = env_url.as_str();

    #[cfg(not(debug_assertions))]
    let db_url: &'static str = std::env!("DATABASE_URL");

    SqliteConnection::establish(db_url).expect("Error while connecting to database!")
}
