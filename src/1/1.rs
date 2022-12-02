use std::fs;

fn main() {
    let lines = read_lines();

    let max: i32 = lines
        .into_iter()
        .map(|s| sum(s))
        .max()
        .expect("Expected value");

    println!("{max}")
}

fn read_lines() -> Vec<String> {
    let contents: String = fs::read_to_string("input.txt").expect("Error reading file");
    contents.split("\n\n").map(|s| s.to_string()).collect()
}

fn sum(numbers: String) -> i32 {
    numbers.split("\n").map(|s| s.parse::<i32>().unwrap()).sum()
}
