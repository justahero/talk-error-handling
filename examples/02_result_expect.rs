fn main() {
    let url = std::env::var("DATABASE_URL").expect("Env var 'DATABASE_URL' unset.");
    println!("DATABASE_URL: {}", url);
}
