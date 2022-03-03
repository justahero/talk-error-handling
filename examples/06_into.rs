use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    ParseFailed,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MyError::ParseFailed => "Failed to parse number.",
        };
        write!(f, "{}", s)
    }
}

impl From<ParseIntError> for MyError {
    fn from(_err: ParseIntError) -> Self {
        MyError::ParseFailed
    }
}

fn parse_number(input: &str) -> Result<i32, MyError> {
    Ok(input.parse::<i32>()?)
}

fn main() {
    println!("Result is: {:?}", parse_number("1"));
}
