fn main() {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => panic!("Env var 'DATABASE_URL' is not set."),
    };

    println!("DATABASE_URL: {}", database_url);
}
