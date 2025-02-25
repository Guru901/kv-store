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

    let contents = fs::read_to_string("./data.json").expect("Failed to read file");
    let mut json = JSONParser::from(&contents)
        .expect("Failed to parse JSON")
        .as_object()
        .expect("JSON is not an object")
        .clone();

    json.insert(key, jsonparser::JSONValue::String(value.to_string()));

    fs::write("./data.json", json.to_string()).expect("Failed to write file");
}

fn get_data_from_file(key: &str) -> String {
    check_file_exists();

    let contents = fs::read_to_string("./data.json").expect("Failed to read file");
    let json = JSONParser::from(&contents).expect("Failed to parse JSON");
    let value = json.get(key).expect("Key not found");

    value.as_str().expect("Value is not a string").to_string()
}

#[cfg(test)]
mod tests {
    /*
        **NOTE:** Tests manipulate files and should be run in a single threaded
        environment. To do this, run the following command:

        cargo test -- --test-threads=1
    */
    use super::*;
    use std::fs;

    fn cleanup() {
        if fs::metadata("./data.json").is_ok() {
            fs::remove_file("./data.json").unwrap();
        }
    }

    #[test]
    fn test_add_single_value() {
        cleanup();
        add_data_to_file("key", "value");

        let contents = fs::read_to_string("./data.json").unwrap();
        let json = JSONParser::from(&contents).unwrap();
        let value = json.get("key").unwrap();

        assert_eq!(value.as_str().unwrap(), "value");
        cleanup();
    }

    #[test]
    fn test_get_single_value() {
        cleanup();
        add_data_to_file("key", "value");

        let value = get_data_from_file("key");

        assert_eq!(value, "value");
        cleanup();
    }

    #[test]
    fn test_multiple_key_values() {
        cleanup();
        add_data_to_file("key1", "value1");
        add_data_to_file("key2", "value2");
        add_data_to_file("key3", "value3");

        assert_eq!(get_data_from_file("key1"), "value1");
        assert_eq!(get_data_from_file("key2"), "value2");
        assert_eq!(get_data_from_file("key3"), "value3");
        cleanup();
    }

    #[test]
    fn test_overwrite_existing_value() {
        cleanup();
        add_data_to_file("key", "old_value");
        add_data_to_file("key", "new_value");

        assert_eq!(get_data_from_file("key"), "new_value");
        cleanup();
    }

    #[test]
    fn test_special_characters() {
        cleanup();
        add_data_to_file("special!@#$", "value with spaces");
        add_data_to_file("unicode_key_🦀", "unicode_value_⭐");

        assert_eq!(get_data_from_file("special!@#$"), "value with spaces");
        assert_eq!(get_data_from_file("unicode_key_🦀"), "unicode_value_⭐");
        cleanup();
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_get_nonexistent_key() {
        cleanup();
        get_data_from_file("nonexistent_key");
    }

    #[test]
    fn test_empty_file_initialization() {
        cleanup();
        check_file_exists();

        assert!(fs::metadata("./data.json").is_ok());
        let contents = fs::read_to_string("./data.json").unwrap();
        assert_eq!(contents, "{}");
        cleanup();
    }
}
