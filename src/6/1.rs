use std::collections::HashSet;
use std::fs;

const MARKER_SIZE: usize = 4;

fn main() {
    let chars: Vec<char> = read_input().chars().collect();

    for i in (MARKER_SIZE - 1)..chars.len() {
        let unique: HashSet<char> = chars[(i - MARKER_SIZE + 1)..=i].iter().cloned().collect();
        if unique.len() == MARKER_SIZE {
            println!("{}", i + 1);
            break;
        }
    }
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}
