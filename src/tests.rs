#[cfg(test)]
mod tests {
    /*
        **NOTE:** Tests manipulate files and should be run in a single threaded
        environment. To do this, run the following command:

        cargo test -- --test-threads=1
    */
    use crate::{add_data_to_file, check_file_exists, get_data_from_file};
    use jsonparser::JSONParser;
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
    fn test_get_nonexistent_key() {
        cleanup();
        let value = get_data_from_file("nonexistent_key");
        assert_eq!(value, "");
        cleanup();
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

    #[test]
    fn test_long_key_value() {
        cleanup();
        let long_key = "k".repeat(1000);
        let long_value = "v".repeat(1000);
        add_data_to_file(&long_key, &long_value);

        assert_eq!(get_data_from_file(&long_key), long_value);
        cleanup();
    }

    #[test]
    fn test_empty_strings() {
        cleanup();
        add_data_to_file("", "empty_key");
        add_data_to_file("empty_value", "");

        assert_eq!(get_data_from_file(""), "empty_key");
        assert_eq!(get_data_from_file("empty_value"), "");
        cleanup();
    }

    #[test]
    fn test_json_structure_preservation() {
        cleanup();
        // First add some data
        add_data_to_file("key1", "value1");
        add_data_to_file("key2", "value2");

        // Manually verify the JSON structure
        let contents = fs::read_to_string("./data.json").unwrap();
        assert!(contents.starts_with("{"));
        assert!(contents.ends_with("}"));
        assert!(contents.contains("{\"key1\": \"value1\", \"key2\": \"value2\"}"));
        cleanup();
    }

    #[test]
    fn test_file_corruption_recovery() {
        cleanup();
        fs::write("./data.json", "invalid json").unwrap();

        add_data_to_file("key", "value");
        assert_eq!(get_data_from_file("key"), "value");
        cleanup();
    }

    #[test]
    fn test_whitespace_keys() {
        cleanup();
        add_data_to_file("   spaced_key   ", "value");
        add_data_to_file("key_with_\t_tab", "value2");
        add_data_to_file("key_with_\n_newline", "value3");

        assert_eq!(get_data_from_file("   spaced_key   "), "value");
        assert_eq!(get_data_from_file("key_with_\t_tab"), "value2");
        assert_eq!(get_data_from_file("key_with_\n_newline"), "value3");
        cleanup();
    }
}
