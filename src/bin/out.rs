use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::fs;
use task_rs::get_db;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_db_migrations(conn: &mut impl MigrationHarness<diesel::sqlite::Sqlite>) {
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Error while running migrations!");
}

fn main() {
    let mut conn = get_db();
    run_db_migrations(&mut conn);

    fs::copy("./taskrs.db", "./out/taskrs.db").unwrap();
    fs::copy("./target/build/release/task-rs", "./out/task-rs").unwrap();
}
