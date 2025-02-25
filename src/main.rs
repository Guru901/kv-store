use kv_store::run;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let command = args.get(1).unwrap_or_else(|| {
        println!("Usage: kv-store-json <command> <key> <value>");
        println!("Commands:");
        println!("  add <key> <value>   Add a key-value pair to the file");
        println!("  get <key>           Get the value of a key");
        std::process::exit(1);
    });

    run(command.as_str(), &args);
}
