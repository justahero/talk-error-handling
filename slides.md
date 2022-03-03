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
  code.language-rust, code.language-shell {
    font-size: 24px;
  }
marp: true
---

# **Error Handling in Rust**

### An Introduction

<!-- ---

# What is an Error? -->

---

# Errors

* recoverable vs unrecoverable
* must be explicitly handled
* transparent error propagation
* vs Java, Ruby, Python, C++, JavaScript etc

---

# Error Handling

* to prevent invalid, undefined or unrecoverable program state
* to recover in an automated way
* to provide users sufficient information
  * e.g. `400 Bad Request`, required CLI argument missing
* to provide developers & operators sufficient context
  * e.g. Postgres unavailable, failed to send email

---

## `panic!`

- macro for unrecoverable errors
- or for "should never happen" scenarios
- exits program
  - unwinds the call stack
  - cleans up data
- helper macros `todo!`, `unimplemented!`, `unreachable!`

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

## `panic!` backtrace

```shell
    Finished dev [unoptimized + debuginfo] target(s) in 6.34s
     Running `target/debug/playground`
thread 'main' panicked at 'Env var 'DATABASE_URL' is not set.', src/main.rs:4:19
stack backtrace:
   0: rust_begin_unwind
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/std/src/panicking.rs:498:5
   1: core::panicking::panic_fmt
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
   2: playground::main
             at ./src/main.rs:4:19
   3: core::ops::function::FnOnce::call_once
             at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ops/function.rs:227:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

---

## `Result` Type

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- enum for recoverable errors
- `Ok(T)` contains success value
- `Err(E)` contains error value
- both `T` & `E` are generic type parameters

---

## Explicit Handling

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
        Ok(value) => { /* Success case */ },
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

- returns success value when `Result` is `Ok`
- panics when `Result` is `Err`
- very limited diagnostics
- useful in only a few situations, e.g. tests

[▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=56d90b0c6c5dbf018d3f4fcaa51543df)

---

## `Result#expect`

```rust
fn main() {
    let url = std::env::var("DATABASE_URL")
        .expect("Env var 'DATABASE_URL' unset.");
    println!("DATABASE_URL: {}", url);
}
```

- panics when `Result` is `Err` variant
- slightly better diagnostics

[▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=558e3a4412d8033c00e5cd6bb1ad21af)

---

# Error Types

* Layered error handling is complex, e.g. application vs crate level
* Rust errors, e.g. `std::io::Error`, `std::num::ParseIntError`
* Domain & crate errors, e.g. `400 Bad Request`, `sqlx::Error`
* Involves error conversion & error chaining

---

## The `std::error::Error` Trait

```rust
pub trait Error: Debug + Display {
    // ...
}
```

- generic trait to represent errors
- useful to provide diagnostics
  - for developers & operators (`Debug`)
  - for users (`Display`)

---

## Error Conversion

```rust
fn parse_number(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    input.parse::<i32>().map_err(|err| From::from(err))
}
```

- `Box<dyn Error>` is opaque
- `ParseIntError` implements `Error` trait
- `From<ParseIntError>` & `Into<Error>` are equivalent

---

## Error Conversion

```rust
fn parse_number(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(input.parse::<i32>()?)
}
```

- `?` operator applies conversion using `Into<Error>`
- unpacks success value or returns function with `Err` value
- equivalent version to last slide

[▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=973ff56797f9e0149c32e721d2d7a1e3)

---

## Error Conversion

```rust
fn parse_from_file(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file_path)?;
    let number = content.parse::<i32>()?;
    Ok(number)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Number in file is: {}", parse_from_file("number.txt")?);
    Ok(())
}
```

- `main` can also return `Result` [▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7c95373998861190ed3624cb8a68d0dd)

---

## Custom Error Types: `enum`

```rust
#[derive(Debug)]
enum MyError {
    ParseFailed,
    FileNotFound(PathBuf),
    Generic { reason: String, source: Box<dyn std::error::Error> }
}
```

- an `enum` provides fine granularity
- each variant can represent one error case

---

<style scoped>
code.language-rust {
    font-size: 22px;
}
</style>

## Custom Error Types: `struct`

```rust
#[derive(Debug)]
pub struct MyParseIntError {
    pub input: String,
    pub source: ParseIntError,
}

impl Display for MyParseIntError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Parse error '{}' with: {}", self.input, self.source)
    }
}
```

- a `struct` or tuple struct also work fine [▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7001658ab6e79b9ce162d656b431db86)

---

## Custom Error Exercise

```rust
fn parse_number(input: &str) -> Result<i32, MyError> {
    todo!("parse input, return number or suitable error variant")
}

fn main() {
    match parse_number("10") {
        Ok(number) => println!("Parsed number is: {}", number),
        Err(err) => println!("Failed to parse number: {}", err),
    }
}
```

- [implement `parse_number` ▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4e332a5a54ef0d9aced532aff46e4532)

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

- maps one `Err` type to another error type [▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4b14efaa5c7544481ba2b0aa0da82cf0)

---

<style scoped>
code.language-rust {
    font-size: 20px;
}
</style>

## Using `Into<E>`

```rust
impl From<ParseIntError> for MyError {
    fn from(_err: ParseIntError) -> Self {
        MyError::ParseFailed
    }
}

fn parse_number(input: &str) -> Result<i32, MyError> {
    Ok(input.parse::<i32>()?)
}

fn main() {
    println!("Result is: {:?}", parse_number("10"));
}
```

- `MyError` implements `From<ParseIntError>` [▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=bfb3df30f5c2c510ec5f702c4aa3f66d)

---

<style scoped>
code.language-rust {
    font-size: 17px;
}
</style>

## Exercise!

```rust
#[derive(Debug)]
pub struct MyParseIntError {
    pub input: String,
    pub source: ParseIntError,
}

impl Display for MyParseIntError { /* */ }

fn parse_number(input: &str) -> Result<i32, MyParseIntError> {
    todo!()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Result is: {:?}", parse_number("10")?);
    Ok(())
}
```

- [Run the example & fix it ▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=71742584930903ad6c3e4018ab345682)

---

## Type Aliases

```rust
type MyResult<T> = std::result::Result<T, MyError>

fn load_file(file_path: &str) -> MyResult<String> { /* */ }
```

- for example `io::Result` in Rust (std)
- can reduce repetition

---

<style scoped>
code.language-rust {
    font-size: 20px;
}
</style>

## Bonus: In iterators

```rust
fn main() {
    let result = vec!["1", "2", "a", "b", "3"]
        .iter()
        .map(|&s| s.parse::<i32>())
        .collect::<Vec<_>>();
    println!("Result: {:?}", result);
}
```

- `Vec<_>` expands to `Vec<Result<i32, ParseIntError>>`
- closure in `map` prohibits to return with `Err` early
- iterates over all items

[▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f47c1c0a616155357b8bbdc09c9a39f4)

---

<style scoped>
code.language-rust {
    font-size: 20px;
}
</style>

## Bonus: In iterators

```rust
fn main() {
    let result = vec!["1", "2", "a", "b", "3"]
        .iter()
        .map(|&s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>();
    println!("Result: {:?}", result);
}
```

- iteration immediately halts at first `Err`
- improves error handling

[▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1acda33948d8118c442d831fd4a59d90)

---

## Useful Crates

- [anyhow](https://github.com/dtolnay/anyhow) to provide an opaque error type
- [thiserror](https://crates.io/crates/thiserror) provides macros to reduce boilerplate code
- [tracing](https://docs.rs/tracing/latest/tracing/) framework to collect structured diagnostics

---

## References

- [Error Handling in Rust](https://www.youtube.com/watch?v=jpVzSse7oJ4) by Luca Palmieri
- [Rust Error Handling](https://fettblog.eu/rust-error-handling/) by Stefan Baumgartner
- [Error Handling in Rust By Example](https://doc.rust-lang.org/rust-by-example/error.html)
- [Rust: Structuring and handling errors in 2020](https://nick.groenen.me/posts/rust-error-handling/) by Nick Groenen

---

<!-- _class: lead -->

# **Questions?**

---

<!-- _class: lead -->

# **Thank You :)**
