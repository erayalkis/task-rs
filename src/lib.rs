pub mod helpers;
pub mod models;
pub mod schema;

use diesel::{sqlite::SqliteConnection, Connection};
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static DB: Mutex<Lazy<SqliteConnection>> = Mutex::new(Lazy::new(|| get_db()));

pub fn get_db() -> SqliteConnection {
    // TODO: Replace this to use the `dotenv` package instead
    let db_url = "./taskrs.db";

    println!("Creating DB connection!");
    SqliteConnection::establish(db_url).expect("Error while connecting to database!")
}
