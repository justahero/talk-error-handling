fn main() {
    let input = "10";
    if input.parse::<i32>().is_ok() {
        println!("Input was a number");
    }
}
