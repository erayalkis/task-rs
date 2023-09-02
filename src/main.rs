use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    command: String,
    #[arg(short, long)]
    list_name: String,
}

fn main() {
    let args = Args::parse();

    match args.command.as_str() {
        "ls" => {}
        "edit" => {}
        "toggle" => {}
        _ => {}
    }
}
