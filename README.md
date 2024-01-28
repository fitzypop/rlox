# rlox

rlox - Writing lox from Crafting Interpreters book in Rust

Current Chapter: https://craftinginterpreters.com/scanning.html

## Tips working with anyhow

```rust
// Create Custom Error, can be struct or Enum
// Struct must impl Debug and Display
#[derive(Debug)]
struct CustomError;

impl std::fmt::Display for CustomError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Something happened")
  }
}

// Use customError
fn do_something() -> anyhow::Result<()> {
  Err(anyhow!(CustomError))
}

// enum example
#[derive(Debug)]
enum MyErrors {
    FileNotFound { name: String },
    FileRead { name: String, msg: String },
    UnknownArg { input: String },
}

impl std::fmt::Display for MyErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound { name } => write!(f, "File not found: {:?}", name),
            Self::FileRead { name, msg } => write!(f, "Error reading file {:?}: {:?}", name, msg),
            Self::UnknownArg { input } => write!(f, "Unknown args: {:?}", input),
        }
    }
}
```
