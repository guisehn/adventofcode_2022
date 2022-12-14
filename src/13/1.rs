//! ```cargo
//! [dependencies]
//! json = "0.12.4"
//! ```

use json::array;
use json::JsonValue;
use std::fs;

fn main() {
    let input = read_input();
    let pairs = parse_pairs(&input);
    let mut sum = 0;

    for (i, (a, b)) in pairs.iter().enumerate() {
        match is_ordered(&a, &b) {
            Some(true) => {
                // println!("Pair {}", i + 1);
                sum += i + 1;
            }

            _ => (),
        };
    }

    dbg!(&sum);
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn parse_pairs(input: &str) -> Vec<(JsonValue, JsonValue)> {
    input
        .trim()
        .split("\n\n")
        .map(|pair| match pair.split_once('\n') {
            Some((a, b)) => (json::parse(a).unwrap(), json::parse(b).unwrap()),
            _ => panic!("Invalid pair"),
        })
        .collect()
}

fn is_ordered(first_list: &JsonValue, second_list: &JsonValue) -> Option<bool> {
    for i in 0..second_list.len() {
        let a = &first_list[i];
        let b = &second_list[i];

        // first list ended first: ordered
        if a.is_null() {
            return Some(true);
        }

        if a.is_number() && b.is_number() {
            let aa = first_list[i].as_i32();
            let bb = second_list[i].as_i32();

            if aa < bb {
                return Some(true);
            } else if aa > bb {
                return Some(false);
            }

            continue;
        }

        if a.is_array() && b.is_array() {
            match is_ordered(&a, &b) {
                Some(value) => return Some(value),
                None => (),
            }

            continue;
        }

        if a.is_array() != b.is_array() {
            let aa = to_array(&a);
            let bb = to_array(&b);

            match is_ordered(&aa, &bb) {
                Some(value) => return Some(value),
                None => (),
            }

            continue;
        }

        panic!("Unexpected scenario");
    }

    // second list ended first: not ordered
    if first_list.len() > second_list.len() {
        return Some(false);
    }

    None
}

fn to_array(value: &JsonValue) -> JsonValue {
    if value.is_array() {
        value.clone()
    } else {
        array![value.clone()]
    }
}
