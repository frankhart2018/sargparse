# Documentation for SArgparse

You are just three steps away from simple argument parsing:

1. Create an instance of ArgumentParser with an optional description, the `new()` method takes an instance of `Option<&str>` as input, which can either be set to `None` or `Some(...)` depending on whether you need a description or not:

```rust
let parser = ArgumentParser::new(None);
```

```rust
let parser = ArgumentParser::new(Some("My awesome program"));
```