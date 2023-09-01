use clap::Parser;
use diesel::prelude::*;
use task_rs::get_db;
use task_rs::models::*;

#[derive(Parser, Debug)]
struct Args {
    todo_body: String,
    #[arg(short, long)]
    list_name: String,
}

fn main() {
    use task_rs::schema::lists::dsl::*;

    let args = Args::parse();

    let conn = &mut get_db();
    let res = lists.limit(5).select(List::as_select()).load(conn).unwrap();

    for list in res {
        println!("ID: {} | TITLE: {}", list.id, list.title);
    }
}
