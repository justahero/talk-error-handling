#[derive(Debug)]
enum MyError {
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

fn load_and_parse(file_path: &str) -> Result<i32, MyError> {
    let content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => return Err(MyError::FileNotLoaded),
    };
    match content.parse::<i32>() {
        Ok(value) => Ok(value),
        Err(_) => Err(MyError::ParseFailed),
    }
}

fn main() {
    match load_and_parse("number.txt") {
        Ok(value) => println!("The number is: {}", value),
        Err(err) => println!("There was an error: {}", err),
    }
}
