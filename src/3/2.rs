use std::collections::HashSet;
use std::fs;

fn main() {
    let lines = read_input();

    let mut sum = 0;

    for group in lines.chunks(3) {
        let item = get_common_item(group);
        let priority = get_priority(item);
        sum += priority;
    }

    println!("{sum}");
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn get_common_item(group: &[String]) -> char {
    let set1: HashSet<_> = group[0].chars().collect();
    let set2: HashSet<_> = group[1].chars().collect();
    let set3: HashSet<_> = group[2].chars().collect();
    let common: HashSet<_> = set1.intersection(&set2).map(|s| *s).collect();
    *set3.intersection(&common).next().unwrap()
}

fn get_priority(item: char) -> u32 {
    let subtract = if item.is_uppercase() { 38 } else { 96 };
    return (item as u32) - subtract;
}
