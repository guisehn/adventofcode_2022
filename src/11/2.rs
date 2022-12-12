//! ```cargo
//! [dependencies]
//! regex = "1"
//! lazy_static = "1.4.0"
//! ```
//!
use std::collections::VecDeque;
use std::fmt;
use std::fs;

// TIL: The 'a lifetime is needed because of the closures.
//
// The closures `operation` and `send` have access to variables defined
// in their context (i.e. that are not their arguments). See that build
// the contexts for more info.
//
// From what I understood, annotating with the lifetime operator is
// needed for the compiler to know that the lifetime of these external
// values is the same lifetime of the Monkey struct instance.
pub struct Monkey<'a> {
    inspect_count: u64,
    division: u64,
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64 + 'a>,
    send: Box<dyn Fn(u64) -> u64 + 'a>,
}

impl fmt::Debug for Monkey<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Monkey<items: {:?}, inspect_count: {}>",
            self.items, self.inspect_count
        )
    }
}

mod monkey_builder {
    use crate::Monkey;
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::collections::VecDeque;

    pub fn build(input: &str) -> Monkey {
        let items = build_items(input);
        let operation = build_operation(input);
        let division = build_division(input);
        let send = build_send(input, division);

        Monkey {
            items,
            operation,
            send,
            inspect_count: 0,
            division: division,
        }
    }

    fn build_items(input: &str) -> VecDeque<u64> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Starting items: (.+)").unwrap();
        }

        let cap = RE.captures(input).unwrap();
        cap[1].split(", ").map(|x| x.parse().unwrap()).collect()
    }

    fn build_operation(input: &str) -> Box<dyn Fn(u64) -> u64> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"Operation: new = (old|[0-9]+) ([*+]) (old|[0-9]+)").unwrap();
        }

        let operate = |a: u64, b: u64, operation: char| match operation {
            '+' => a + b,
            '*' => a * b,
            _ => panic!("Unknown operation {}", operation),
        };

        let cap = RE.captures(input).unwrap();
        let operator = cap[2].chars().next().unwrap();

        match (&cap[1], &cap[3]) {
            ("old", "old") => Box::new(move |old| operate(old, old, operator)),
            (value, "old") => {
                let num: u64 = value.to_owned().parse().unwrap();
                Box::new(move |old| operate(num, old, operator))
            }
            ("old", value) => {
                let num: u64 = value.to_owned().parse().unwrap();
                Box::new(move |old| operate(old, num, operator))
            }
            (&_, &_) => todo!(),
        }
    }

    fn build_division(input: &str) -> u64 {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Test: divisible by ([0-9]+)").unwrap();
        }

        let cap = RE.captures(input).unwrap();
        cap[1].to_owned().parse().unwrap()
    }

    fn build_send(input: &str, divisible_by: u64) -> Box<dyn Fn(u64) -> u64> {
        lazy_static! {
            static ref RE_TRUE: Regex = Regex::new(r"If true: throw to monkey ([0-9]+)").unwrap();
            static ref RE_FALSE: Regex = Regex::new(r"If false: throw to monkey ([0-9]+)").unwrap();
        }

        let cap_true = RE_TRUE.captures(input).unwrap();
        let true_monkey: u64 = cap_true[1].to_owned().parse().unwrap();

        let cap_false = RE_FALSE.captures(input).unwrap();
        let false_monkey: u64 = cap_false[1].to_owned().parse().unwrap();

        Box::new(move |value| {
            let result = if value % divisible_by == 0 {
                true_monkey
            } else {
                false_monkey
            };

            result
        })
    }
}

fn main() {
    let input = read_input();

    let mut monkeys: Vec<Monkey> = input
        .trim()
        .split("\n\n")
        .map(|item| monkey_builder::build(item))
        .collect();

    // Thanks Reddit for the trick
    let modulo: u64 = monkeys.iter().map(|m| m.division).product();

    for _round in 0..10_000 {
        // I can't do `for monkey in &mut monkeys` because the compiler
        // complains when I try to move the item to another monkey, due
        // to a duplicate mutable borrow, so we need to go with indexes.
        // Can't store the current monkey in a variable either to avoid
        // repeating monkey_index over and over.
        // Rust is annoying!
        for monkey_index in 0..monkeys.len() {
            while monkeys[monkey_index].items.len() > 0 {
                monkeys[monkey_index].inspect_count += 1;

                let mut item = monkeys[monkey_index].items.pop_front().unwrap();
                item = (monkeys[monkey_index].operation)(item);
                item %= modulo;

                let target = (monkeys[monkey_index].send)(item);
                monkeys[target as usize].items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    dbg!(&monkeys);

    let result: u64 = monkeys[0..2]
        .iter()
        .map(|monkey| monkey.inspect_count)
        .product();

    println!("Result: {result}");
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}
