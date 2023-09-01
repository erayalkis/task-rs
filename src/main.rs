use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    todo_body: String,
    #[arg(short, long)]
    list_name: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.todo_body);
    println!("{}", args.list_name);
}
