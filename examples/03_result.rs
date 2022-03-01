fn division(divident: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        Err("Divisor is zero".to_string())
    } else {
        Ok(divident / divisor)
    }
}

fn main() {
    println!("Result is: {:?}", division(4, 0));
}