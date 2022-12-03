use std::fs;

fn main() {
    let lines = read_lines();

    let mut elves = lines.into_iter().map(|s| sum(s)).collect::<Vec<i32>>();

    elves.sort_by(|a, b| b.cmp(a));

    let three_most = &elves[..3].into_iter().sum::<i32>();

    println!("{three_most}")
}

fn read_lines() -> Vec<String> {
    let contents: String = fs::read_to_string("input.txt").unwrap();
    contents.split("\n\n").map(|s| s.to_string()).collect()
}

fn sum(numbers: String) -> i32 {
    numbers.split("\n").map(|s| s.parse::<i32>().unwrap()).sum()
}
