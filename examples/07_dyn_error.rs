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

impl std::error::Error for MyError {}

fn parse_number(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    input.parse::<i32>().map_err(|_| Box::new(MyError::ParseFailed))
}

fn main() {
    match parse_number("a") {
        Ok(number) => println!("Parsed number is: {}", number),
        Err(err) => println!("{}", err),
    }
}
