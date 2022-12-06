//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

use regex::Regex;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Input {
    stacks: Vec<VecDeque<char>>,
    moves: Vec<Move>,
}

fn main() {
    let Input { mut stacks, moves } = parse_input(&read_input());

    for movement in moves {
        let items: VecDeque<char> = stacks[movement.from - 1]
            .drain(0..movement.quantity)
            .rev()
            .collect();

        for item in items {
            stacks[movement.to - 1].push_front(item);
        }
    }

    let stack_top: String = stacks.iter().map(|x| x[0]).collect();
    println!("{stack_top}");
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn parse_input(input: &str) -> Input {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let stacks = parse_stacks(&sections[0]);
    let moves = parse_moves(&sections[1]);

    return Input {
        stacks: stacks,
        moves: moves,
    };
}

fn parse_stacks(input: &str) -> Vec<VecDeque<char>> {
    let mut lines: Vec<&str> = input.split("\n").collect();
    let quantity: usize = count_stacks(lines.pop().unwrap());

    let mut stacks = vec![VecDeque::<char>::new(); quantity];

    for line in lines {
        let chars: Vec<char> = line.chars().collect();

        for n in 0..quantity {
            let char_index = (n * 4) + 1;
            let item = chars[char_index];
            if item != ' ' {
                stacks[n].push_back(item);
            }
        }
    }

    stacks
}

fn count_stacks(stack_numbers: &str) -> usize {
    return stack_numbers
        .trim()
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap();
}

fn parse_moves(input: &str) -> Vec<Move> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    re.captures_iter(input)
        .map(|cap| Move {
            quantity: cap[1].parse::<usize>().unwrap(),
            from: cap[2].parse::<usize>().unwrap(),
            to: cap[3].parse::<usize>().unwrap(),
        })
        .collect()
}
