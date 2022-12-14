//! ```cargo
//! [dependencies]
//! json = "0.12.4"
//! ```

use json::array;
use json::JsonValue;
use std::cmp::Ordering;
use std::fs;

fn main() {
    let mut input = read_input();
    input.push_str("\n[[2]]\n[[6]]");

    let mut lists = parse_lists(&input);
    lists.sort_by(|a, b| cmp(a, b));

    let mut result = 1;
    for (i, list) in lists.iter().enumerate() {
        if &list.dump() == "[[2]]" || &list.dump() == "[[6]]" {
            result *= i + 1;
        }
    }

    dbg!(&result);
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn parse_lists(input: &str) -> Vec<JsonValue> {
    input
        .trim()
        .lines()
        .filter(|line| !line.trim().eq(""))
        .map(|line| json::parse(line).unwrap())
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

fn cmp(first_list: &JsonValue, second_list: &JsonValue) -> Ordering {
    match is_ordered(&first_list, &second_list) {
        Some(true) => Ordering::Less,
        None => Ordering::Equal,
        Some(false) => Ordering::Greater,
    }
}

fn to_array(value: &JsonValue) -> JsonValue {
    if value.is_array() {
        value.clone()
    } else {
        array![value.clone()]
    }
}
