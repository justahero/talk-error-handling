fn parse_from_file(file_path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file_path)?;
    let number = content.parse::<i32>()?;
    Ok(number)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Number in file is: {}", parse_from_file("number.txt")?);
    Ok(())
}
