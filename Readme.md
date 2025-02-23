# KV Store JSON

A simple key-value store implemented in Rust that stores data in a JSON file. This project allows you to add and retrieve key-value pairs using command-line arguments.

## Features

- Add a key-value pair to a JSON file.
- Retrieve the value of a given key from the JSON file.
- Automatically creates a JSON file if it does not exist.

## Installation

1. Ensure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone the repository:

   ```bash
   git clone <repository-url>
   cd kv-store
   ```

3. Build the project:

   ```bash
   cargo build
   ```

## Usage

You can run the program using the following commands:

- To add a key-value pair:

  ```bash
  cargo run add <key> <value>
  ```

- To get the value of a key:

  ```bash
  cargo run get <key>
  ```

## Example

- To add a key-value pair:

  ```bash
  cargo run add key value
  ```

- To get the value of a key:

  ```bash
  cargo run get key
  ```

## Testing

To run the tests, use the following command:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
