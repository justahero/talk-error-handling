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
marp: true
---

# **Error Handling in Rust**

### An Introduction

---

# Errors

- recoverable vs unrecoverable
- explicit handling
- no exceptions
- vs Java, Ruby, Python, C++

---

## `panic!`

- unrecoverable error
- or for "should never happen" scenarios
- exits program
  - unwinds the call stack
  - cleans up data

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

## Result

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

## Result Example

```rust
fn main() {
    "10".parse::<i32>();
}
```

```bash
warning: unused `Result` that must be used
 --> src/main.rs:2:5
  |
2 |     "10".parse::<i32>();
  |     ^^^^^^^^^^^^^^^^^^^^
  |
```

[▶️](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a275602f885e05e01ad5f95b3160f078)

---

## `std::error::Error` trait

```rust
pub trait Error: Debug + Display {
    // ...
}
```

- generic trait to display error

---

## Custom Error type

```rust
#[derive(Debug)]
enum CustomError {
    ParseFailed,
    FileNotLoaded,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MyError::ParseFailed => "Failed to parse number.",
            MyError::FileNotLoaded => "Failed to load file.",
        };
        write!(f, "{}", s)
    }
}
```

---

## Custom Error type

