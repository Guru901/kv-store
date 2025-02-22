use std::env;
use std::fs;
use std::process::Command;

use jsonparser::JSONParser;

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

    match JSONParser::from(&contents) {
        Ok(json) => println!("{:#?}", json.get("hehe").unwrap()),
        Err(e) => eprintln!("Failed to parse JSON: {}", e),
    }

    match command.as_str() {
        "add" => {
            let key = args.get(2).unwrap();
            let value = args.get(3).unwrap();
            // add this data to file
        }
        "get" => {
            let key = args.get(2).unwrap();
            // get this data from file
        }
        _ => {
            println!("Usage: kv-store-json <command> <key> <value>");
            println!("Commands:");
            println!("  add <key> <value>   Add a key-value pair to the file");
            println!("  get <key>           Get the value of a key");
        }
    }
}
