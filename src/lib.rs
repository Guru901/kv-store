use jsonparser::{JSONParser, JSONValue, OrderedMap};
use std::fs;

fn check_file_exists() {
    if !fs::metadata("./data.json").is_ok() {
        fs::write("./data.json", r#"{}"#).unwrap();
    }
}

pub fn run(command: &str, args: &Vec<String>) {
    match command {
        "add" => {
            let key = args.get(2).unwrap();
            let value = args.get(3).unwrap();

            add_data_to_file(key, value);
        }
        "get" => {
            let key = args.get(2).unwrap();
            let value = get_data_from_file(key);
            println!("{}", value)
        }
        _ => {
            println!("Usage: kv-store-json <command> <key> <value>");
            println!("Commands:");
            println!("  add <key> <value>   Add a key-value pair to the file");
            println!("  get <key>           Get the value of a key");
        }
    }
}

fn add_data_to_file(key: &str, value: &str) {
    check_file_exists();

    let contents = fs::read_to_string("./data.json").expect("Failed to read file");
    let json = JSONParser::from(&contents);
    let mut json_content = match json {
        Ok(json) => json.as_object().unwrap().clone(),
        Err(_) => {
            fs::write("./data.json", "{}").unwrap();
            OrderedMap::new()
        }
    };
    json_content.insert(key, JSONValue::String(value.to_string()));
    fs::write("./data.json", json_content.to_string()).expect("Failed to write file");
}

fn get_data_from_file(key: &str) -> String {
    check_file_exists();

    let contents = fs::read_to_string("./data.json").expect("Failed to read file");
    let json = JSONParser::from(&contents);

    let json_content = match json {
        Ok(json) => json.as_object().unwrap().clone(),
        Err(_) => {
            fs::write("./data.json", "{}").unwrap();
            OrderedMap::new()
        }
    };

    let value = json_content.get(key);

    match value {
        Some(value) => {
            return value.as_str().expect("Value is not a string").to_string();
        }
        None => {
            println!("Key not found");
            return String::new();
        }
    }
}

pub mod tests;
