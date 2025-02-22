use std::env;
use std::fs;
use std::process::Command;

fn main() {
    Command::new("touch data.json").spawn().unwrap();

    let contents = fs::read_to_string("./test.json").unwrap();

    let args = env::args().collect::<Vec<String>>();

    let command = args.get(1).unwrap_or_else(|| {
        println!("Usage: kv-store-json <command> <key> <value>");
        println!("Commands:");
        println!("  add <key> <value>   Add a key-value pair to the file");
        println!("  get <key>           Get the value of a key");
        std::process::exit(1);
    });
}
