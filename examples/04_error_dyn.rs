fn parse_number(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
    input.parse::<i32>().map_err(|err| err.into())
}

fn main() {
    match parse_number("10") {
        Ok(number) => println!("Parsed number is: {}", number),
        Err(err) => println!("Failed to parse number: {}", err),
    }
}
