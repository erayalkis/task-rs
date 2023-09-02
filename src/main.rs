use clap::Parser;
use task_rs::helpers::{display_list_items, get_list_items};

#[derive(Parser, Debug)]
struct Args {
    command: String,
    #[arg(short, long)]
    list_name: String,
}

fn main() {
    let args = Args::parse();

    match args.command.as_str() {
        "ls" => {
            let content = get_list_items(&args.list_name);
            display_list_items(&args.list_name, &content);
        }
        "edit" => {}
        "toggle" => {}
        _ => {}
    }
}
