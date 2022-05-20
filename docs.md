# Documentation for SArgparse

You are just three steps away from simple argument parsing:

1. Create an instance of ArgumentParser with an optional description, the `new()` method takes an instance of `Option<&str>` as input, which can either be set to `None` or `Some(...)` depending on whether you need a description or not:

```rust
let parser = ArgumentParser::new(None);
```

```rust
let parser = ArgumentParser::new(Some("My awesome program"));
```

2. Add arguments to the parser using `add_argument()` method:

```rust
parser.add_argument("f", "file", "The file to read", true, None, ArgumentType::INT);
```

Ok that was a lot of arguments, let break this up:

1. `f` is the short name for the argument, something which you will pass as `-f` from your command line.
2. `file` is the long name for the argument, something which you will pass as `--file` from your command line.
3. `The file to read` is the description of the argument, which will be shown in the help message.
4. `true` is the required flag, which means that the argument is required to be passed from the command line.
5. `None` is the default value, which means that if you don't pass a value for this, then the argument will not have any value, if you want to set a default value, then you can do so by passing `Some(...)` as the default value.
6. `ArgumentType::INT` is the type of the argument, which is used to parse the value from the command line, this can take the following values:
    a. `ArgumentType::INT`: parses the value as an integer.
    b. `ArgumentType::FLOAT`: parses the value as a float.
    c. `ArgumentType::STR`: parses the value as a string.
    d. `ArgumentType::BOOL`: parses the value as a boolean.

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