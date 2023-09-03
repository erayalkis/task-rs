
# This script builds task-rs from source, runs migrations, copies relevant files to ./out, and copies them into /usr/bin to enable running task-rs from the CLI

DATABASE_URL="/usr/local/bin/taskrs.db" cargo build --release

cargo run --bin out

sudo cp -a out/. /usr/local/bin

