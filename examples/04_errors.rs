fn load_file(file_path: &str) -> std::io::Result<String> {
    match std::fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(err) => Err(err),
    }
}

fn main() {
    match load_file("number.txt") {
        Ok(content) => println!("File content is: {}", content),
        Err(err) => println!("Failed to load file: {}", err),
    }
}
