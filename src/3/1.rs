use std::collections::HashSet;
use std::fs;

fn main() {
    let mut sum = 0;

    for rucksack in read_input().trim().lines() {
        let (c1, c2) = get_compartments(rucksack);
        let common = get_common_item(c1, c2);
        let priority = get_priority(common);
        sum += priority;
    }

    println!("{sum}");
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn get_compartments(rucksack: &str) -> (&str, &str) {
    let half = rucksack.len() / 2;
    (&rucksack[..half], &rucksack[half..])
}

fn get_common_item(c1: &str, c2: &str) -> char {
    let set1: HashSet<_> = c1.chars().collect();
    let set2: HashSet<_> = c2.chars().collect();
    *set1.intersection(&set2).next().unwrap()
}

fn get_priority(item: char) -> u32 {
    let subtract = if item.is_uppercase() { 38 } else { 96 };
    return (item as u32) - subtract;
}
