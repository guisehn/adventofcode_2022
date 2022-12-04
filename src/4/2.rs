//! ```cargo
//! [dependencies]
//! regex = "1"
//! ```

use regex::Regex;
use std::fs;
use std::ops::Range;

fn main() {
    let input = read_input();
    let ranges = get_ranges(&input);
    let overlapping = ranges.iter().filter(|(a, b)| overlaps(a, b)).count();
    println!("{overlapping}");
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn get_ranges(input: &str) -> Vec<(Range<i32>, Range<i32>)> {
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let (a, b, c, d) = (
                to_int(&cap[1]),
                to_int(&cap[2]),
                to_int(&cap[3]),
                to_int(&cap[4]),
            );

            (Range { start: a, end: b }, Range { start: c, end: d })
        })
        .collect()
}

fn to_int(value: &str) -> i32 {
    value.parse::<i32>().unwrap()
}

fn overlaps(a: &Range<i32>, b: &Range<i32>) -> bool {
    (a.end >= b.start && a.start <= b.end) || (b.end >= a.start && b.start <= a.end)
}

// .....A....
// ...BB.....

// 5-7, 7-9
// 2-8, 3-7
// 6-6, 4-6
// 2-6, 4-8