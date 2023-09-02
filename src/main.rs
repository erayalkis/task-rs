use clap::Parser;
use task_rs::helpers::{display_list_items, toggle_task_completion};

#[derive(Parser, Debug)]
struct Args {
    command: String,
    #[arg(short, long)]
    list_name: String,
    task_id: Option<i32>,
}

fn main() {
    let args = Args::parse();

    match args.command.as_str() {
        "ls" => {
            display_list_items(&args.list_name);
        }
        "edit" => {}
        "toggle" => {
            toggle_task_completion(&args.task_id).unwrap();
        }
        _ => {
            if args.list_name.len() == 0 {
                panic!("Cannot create a task without the <LIST_NAME> parameter!");
            }
        }
    }
}
