# Documentation for SArgparse

You are just three steps away from simple argument parsing:

1. Create an instance of ArgumentParser with an optional description, the `new()` method takes an instance of `Option<&str>` as input, which can either be set to `None` or `Some(...)` depending on whether you need a description or not:

```rust
let mut parser = ArgumentParser::new(None);
```

```rust
let mut parser = ArgumentParser::new(Some("My awesome program"));
```

2. Add arguments to the parser using `add_argument()` method:

```rust
parser.add_argument("-f", "--file", "The file to read", true, None, ArgumentType::INT);
```

Ok that was a lot of arguments, let break this up:

1. `f` is the short name for the argument, something which you will pass as `-f` from your command line.
2. `file` is the long name for the argument, something which you will pass as `--file` from your command line.
3. `The file to read` is the description of the argument, which will be shown in the help message.
4. `true` is the required flag, which means that the argument is required to be passed from the command line.
5. `None` is the default value, which means that if you don't pass a value for this, then the argument will not have any value, if you want to set a default value, then you can do so by passing `Some(...)` as the default value. In case there are no default values and the argument is optional, then it will have the default 0 value (0 for INT, 0.0 for FLOAT, false for BOOL, and "" for STRING) value assigned to it.
6. `ArgumentType::INT` is the type of the argument, which is used to parse the value from the command line, this can take the following values:
    a. `ArgumentType::INT`: parses the value as an integer.
    b. `ArgumentType::FLOAT`: parses the value as a float.
    c. `ArgumentType::STR`: parses the value as a string.
    d. `ArgumentType::BOOL`: parses the value as a boolean. Arguments of these types are simply flags and do not take any values.

Arguments can also be positional, positional arguments have to appear in the order in which they were added otherwise it will result in an error. To add a positional argument, simply remove the - before the short name and -- before the long name:

```rust
parser.add_argument("f", "file", "The file to read", true, None, ArgumentType::INT);
```

These values can be passed using the value directly in place.

3. Parse your arguments, get your values using the `parse_args()` method:

```rust
let args = parser.parse_args();
```

This parser values from `std::env::args()` which is the standard way of reading values from command line in Rust.

Once you have the args parsed, you can access them using Rust's `get()` method for HashMap because `parse_args()` returns a HashMap with the key as the `long_name` and the value as the parsed_value:

```rust
let file = args.get("file").unwrap().get_str();
```

Ok, I cheated a bit here, what's with the `get_str()` here, well Rust is a typed language and needs the type of arguments, and I don't want you to use the internal data enum type as your variable types, so you can convert it to your required type using one of `get_int()`, `get_float()`, `get_str()` or `get_bool()` methods depending on the type of the argument.

That's it, you're done! Wasn't this easy? 

# Example

Here's a complete example of how to use this parser:

1. Add `sargparse` as a dependency:

```toml
[dependencies]
sargparse = "0.1"
```

2. The actual program:

```rust
extern crate sargparse;

use sargparse::{ArgumentParser, ArgumentType, InnerData};

fn main() {
    let mut parser = ArgumentParser::new(Some("Awesome program"));

    parser.add_argument("f", "file", "File input for awesome program", 
                        true, None, ArgumentType::STR);
    parser.add_argument("-i", "--input", "Input string for awesome program", 
                        true, None, ArgumentType::STR);
    parser.add_argument("-n", "--number", "Input int for awesome program", 
                        false, Some(InnerData::INT(2)), ArgumentType::INT);
    parser.add_argument("-f", "--float", "Input float for awesome program", 
                        false, Some(InnerData::FLOAT(3.14)), ArgumentType::FLOAT);
    parser.add_argument("-b", "--bool", "Input bool for awesome program", 
                        false, Some(InnerData::BOOL(false)), ArgumentType::BOOL);

    let args = parser.parse_args().unwrap();

    let file = args.get("file").unwrap().get_str();
    let input = args.get("input").unwrap().get_str();
    let number = args.get("number").unwrap().get_int();
    let float = args.get("float").unwrap().get_float();
    let bool = args.get("bool").unwrap().get_bool();

    assert_eq!(file, "sample.txt");
    assert_eq!(input, "sample");
    assert_eq!(number, 25);
    assert_eq!(float, 44.56);
    assert_eq!(bool, true);
}
```

If running using `cargo run`, this can be run using:

```bash
cargo run -- sample.txt -i sample -n 25 -f 44.56 --bool
```

If you are running the binary then forget the -- at the beginning:

```bash
./awesome-program sample.txt -i sample -n 25 -f 44.56 --bool
```