use std::num::ParseIntError;

fn main() {
    let input: Result<i32, ParseIntError> = "10".parse::<i32>();
    match input {
        Ok(value) => println!("Value is: {}", value),
        Err(_) => println!("Failed to parse string."),
    }
}
