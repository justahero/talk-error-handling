---
theme: gaia
_class: lead
paginate: true
backgroundColor: #eee
style: |
  section.lead h1 {
    text-align: center;
  }
  h6 {
    font-size: 30%;
  }
  code.rust, code.shell {
    font-size: 18px;
  }
marp: true
---

# **Error Handling in Rust**

### An Introduction

<!-- ---

# What is an Error? -->

---

# Errors

- recoverable vs unrecoverable
- explicit handling
- no exceptions
- vs Java, Ruby, Python, C++

---

# Goals

- to prevent invalid, undefined or unrecoverable program state
- to provide users with sufficient information
  - e.g. `400 Bad Request`, required CLI argument missing
- to provide developers & operators enough context
  - e.g. `503 Sercice Unavailabe`, env var `X` unset
  - aggregate logs / traces externally

---

## `panic!`

- unrecoverable error
- also for "should never happen" scenarios
- exits program
  - unwinds the call stack
  - cleans up data
- other macros `todo!`, `unimplemented!`, `unreachable!`

---

## `panic!` Example

```rust
fn main() {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => panic!("Env var 'DATABASE_URL' is not set."),
    };

    println!("DATABASE_URL: {}", database_url);
}
```

- run with `RUST_BACKTRACE=1` to display a backtrace

[▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=17bea9b2cf6e08db93ea59fd47305eb1)

---

## `Result` Type

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `Ok(T)` contains success value
- `Err(E)` contains error value
- both `T` & `E` are generic type parameters

---

## Explicit

```rust
fn main() {
    "10".parse::<i32>();
}
```

```shell
warning: unused `Result` that must be used
 --> src/main.rs:2:5
  |
2 |     "10".parse::<i32>();
  |     ^^^^^^^^^^^^^^^^^^^^
  |
```

[▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a275602f885e05e01ad5f95b3160f078)

---

## Using `match`

```rust
fn main() {
    let input: Result<i32, ParseIntError> = "10".parse::<i32>();
    match input {
        Ok(value) => println!("Value is: {}", value),
        Err(_) => println!("Failed to parse string."),
    }
}
```

- `parse` returns either `Ok(i32)` or `Err(ParseIntError)`

[▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=56eac5ffc22b9a23bf7b82b9abbaa2e5)

---

## `is_ok` & `is_err`

```rust
fn main() {
    let input = "10";
    if input.parse::<i32>().is_ok() {
        println!("Input was a number");
    }
}
```

- `Result#is_ok` returns true when `Ok` variant
- `Result#is_err` returns true when `Err` variant

---

## `Result#unwrap`

```rust
fn main() {
    let x = "10".parse::<i32>().unwrap();
    println!("x is: {}", x);
}
```

- panics when `Result` is `Err` variant
- very limited diagnostics
- useful in some limited capacity, e.g. tests

[▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=56d90b0c6c5dbf018d3f4fcaa51543df)

---

## `Result#expect`

```rust
fn main() {
    let url = std::env::var("DATABASE_URL").expect("Env var 'DATABASE_URL' unset.");
    println!("DATABASE_URL: {}", url);
}
```

- panics when `Result` is `Err` variant
- slightly better diagnostics

---

# Error Types

- Layered error handling is complex, e.g. application vs crate
- Rust errors, e.g. `std::io::Error`, `std::num::ParseIntError`
- Domain & crate errors, e.g. `400 Bad Request`, `sqlx::Error`
- Often requires some form of error conversion

---

## Custom Error Types

```rust
#[derive(Debug)]
enum MyError {
    ParseFailed,
}
```

---

## Custom Error Exercise

```rust
fn parse_number(input: &str) -> Result<i32, MyError> {
    // TODO parse input, return number or suitable error variant
}

fn main() {
    match parse_number("10") {
        Ok(number) => println!("Parsed number is: {}", number),
        Err(err) => println!("Failed to parse number: {}", err),
    }
}
```

- [implement `parse_number` ▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c1070dfc7bba26930e866ad67a103ae7)

---

## Using `map_err`

```rust
fn parse_number(input: &str) -> Result<i32, MyError> {
    input.parse::<i32>().map_err(|_| MyError::ParseFailed)
}

fn main() {
    match parse_number("10") {
        Ok(number) => println!("Parsed number is: {}", number),
        Err(err) => println!("{}", err),
    }
}
```

- maps one `Err` type to another error type

---

## `std::error::Error` trait

```rust
pub trait Error: Debug + Display {
    // ...
}
```

- generic trait to display errors
- useful to provide specific diagnostics
  - for developers & operators (`Debug`)
  - for users (`Display`)

---

## Using `std::error::Error`

```rust
#[derive(Debug)]
enum MyError {
    ParseFailed,
}

impl std::error::Error for MyError { /* */ }

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MyError::ParseFailed => "Failed to parse number.",
        };
        write!(f, "{}", s)
    }
}
```

---

## Error conversion

---

## Exercise

---

## In iterators

```rust
fn main() {
    let list = vec!["1", "2", "a", "b", "3"];
    let result = list
        .iter()
        .map(|&s| s.parse::<i32>())
        .collect::<Vec<_>>();

    println!("Result: {:?}", result);
}
```

- `Vec<_>` expands to `Vec<Result<i32, ParseIntError>>`

```shell
Result: [Ok(1), Ok(2), Err(ParseIntError { kind: InvalidDigit }), Err(ParseIntError { kind: InvalidDigit }), Ok(3)]
```

- `map` closure prohibits to return with `Err` early
- iterates over all items

---

## In iterators

```rust
fn main() {
    let list = vec!["1", "2", "a", "b", "3"];
    let result = list
        .iter()
        .map(|&s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>();

    println!("Result: {:?}", result);
}
```

- iteration immediately halts at first `Err`

---

<!-- _class: lead -->

# **Questions?**

---

<!-- _class: lead -->

# **Thank You :)**

---

