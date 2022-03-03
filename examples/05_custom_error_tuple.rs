use std::num::ParseIntError;

#[derive(Debug)]
pub struct MyParseIntError {
    pub input: String,
    pub source: ParseIntError,
}

impl std::fmt::Display for MyParseIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error '{}' with: {}", self.input, self.source)
    }
}

fn parse_number(input: &str) -> Result<i32, MyParseIntError> {
    match input.parse::<i32>() {
        Ok(value) => Ok(value),
        Err(err) => Err(MyParseIntError {
            input: input.to_string(),
            source: err,
        }),
    }
}

fn main() {
    println!("Result is: {:?}", parse_number("10"));
}
