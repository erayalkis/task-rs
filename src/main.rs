use clap::Parser;
use task_rs::helpers::{
    create_list_record, create_task_record, delete_task_record, display_list_items,
    ensure_at_least_one_list_exists, get_list_at, get_list_from_title, toggle_task_completion,
};

#[derive(Parser, Debug)]
struct Args {
    command: String,
    command_body: Option<String>,
    #[arg(short, long)]
    list_name: Option<String>,
    #[arg(short, long)]
    task_id: Option<i32>,
    #[arg(long)]
    list_id: Option<i32>,
}

fn main() {
    let args = Args::parse();

    match args.command.as_str() {
        "create-list" => {
            let unwrapped_body = &args.command_body.unwrap();
            create_list_record(unwrapped_body).unwrap();
        }
        "ls" => {
            ensure_at_least_one_list_exists();

            if args.command_body.is_some() {
                let unwrapped_body = &args.command_body.unwrap();
                display_list_items(unwrapped_body);
                return;
            }

            let first_list = get_list_at(1).unwrap();
            display_list_items(&first_list.title);
        }
        "edit" => {}
        "toggle" => {
            ensure_at_least_one_list_exists();

            if args.command_body.is_none() {
                panic!("The `toggle` command requires a task ID to be provided!");
            }

            let parsed_body = args.command_body.unwrap().parse::<i32>().unwrap();
            toggle_task_completion(parsed_body).unwrap();
        }
        "delete" => {
            ensure_at_least_one_list_exists();

            if args.command_body.is_none() {
                panic!("The `delete` command requires a task ID to be provided!");
            }

            let parsed_body = args.command_body.unwrap().parse::<i32>().unwrap();
            delete_task_record(parsed_body).unwrap();
        }
        _ => {
            ensure_at_least_one_list_exists();

            let list_id;
            if args.command_body.is_none() {
                list_id = 1
            } else {
                let list = get_list_from_title(args.command_body.unwrap()).unwrap();
                list_id = list.id;
            }

            create_task_record(&args.command, list_id).unwrap();
        }
    }
}
