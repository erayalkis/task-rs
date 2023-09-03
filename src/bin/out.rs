use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::{fs, path};
use task_rs::get_db;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_db_migrations(conn: &mut impl MigrationHarness<diesel::sqlite::Sqlite>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Error while running migrations!");
}

fn create_out_dir_if_not_exists() {
    let path = path::Path::new("./out");
    if !path::Path::exists(path) {
        fs::create_dir(path).unwrap()
    }
}

fn main() {
    let mut conn = get_db();
    run_db_migrations(&mut conn);

    create_out_dir_if_not_exists();
    fs::copy("./taskrs.db", "./out/taskrs.db").unwrap();
    // fs::File::create("./out/taskrs.db").unwrap();
    fs::copy("./target/release/task-rs", "./out/taskrs").unwrap();
}
