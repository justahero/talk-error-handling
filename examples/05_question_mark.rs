fn load_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file_path)?)
}

fn main() {
    match load_file("number.txt") {
        Ok(content) => println!("File content is: {}", content),
        Err(err) => println!("Failed to load file: {}", err),
    }
}
