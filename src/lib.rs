pub mod helpers;
pub mod models;
pub mod schema;

use diesel::{sqlite::SqliteConnection, Connection};

pub fn get_db() -> SqliteConnection {
    // TODO: Replace this to use the `dotenv` package instead
    let db_url = "./taskrs.db";

    SqliteConnection::establish(db_url).expect("Error while connecting to database!")
}
