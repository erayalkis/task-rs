use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    todo_body: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.todo_body);
}
