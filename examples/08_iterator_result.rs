fn main() {
    let list = vec!["1", "2", "a", "b", "3"];
    let result = list
        .iter()
        .map(|&s| s.parse::<i32>())
        .collect::<Vec<_>>();

    println!("Result: {:?}", result);
}