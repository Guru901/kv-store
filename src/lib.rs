use jsonparser::JSONParser;
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

    let contents = fs::read_to_string("./data.json").unwrap();
    let mut json = JSONParser::from(&contents)
        .unwrap()
        .as_object()
        .unwrap()
        .clone();

    json.insert(key, jsonparser::JSONValue::String(value.to_string()));

    fs::write("./data.json", json.to_string()).unwrap();
}

fn get_data_from_file(key: &str) -> String {
    check_file_exists();

    let contents = fs::read_to_string("./data.json").unwrap();
    let json = JSONParser::from(&contents).unwrap();
    let value = json.get(key).unwrap();

    value.as_str().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_to_file_test() {
        add_data_to_file("key", "value");

        let contents = fs::read_to_string("./data.json").unwrap();
        let json = JSONParser::from(&contents).unwrap();
        let value = json.get("key").unwrap();

        assert_eq!(value.as_str().unwrap(), "value");
    }

    #[test]
    fn get_from_file_test() {
        add_data_to_file("key", "value");

        let value = get_data_from_file("key");

        assert_eq!(value, "value");
    }
}
