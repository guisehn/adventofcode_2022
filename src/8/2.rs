use std::collections::HashMap;
use std::fs;
use std::iter::Rev;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct TreeMap {
    map: HashMap<(u32, u32), u32>,
    size_x: u32,
    size_y: u32,
}

impl TreeMap {
    fn new() -> TreeMap {
        TreeMap {
            map: HashMap::new(),
            size_x: 0,
            size_y: 0,
        }
    }

    fn from_str(input: &str) -> TreeMap {
        let mut tree_map = TreeMap::new();
        let mut x = 0;
        let mut y = 0;

        for line in input.trim().lines() {
            x = 0;

            for item in line.chars() {
                let height = item.to_digit(10).unwrap();
                tree_map.map.insert((x as u32, y as u32), height);
                x += 1;
            }

            y += 1;
        }

        tree_map.size_x = x as u32;
        tree_map.size_y = y as u32;
        tree_map
    }

    fn is_edge_tree(&self, (x, y): (u32, u32)) -> bool {
        x == 0 || x == self.size_x - 1 || y == 0 || y == self.size_y - 1
    }

    fn get_scenic_score(&self, (x, y): (u32, u32)) -> i32 {
        if self.is_edge_tree((x, y)) {
            return 0;
        }

        let tree_height = self.get((x, y));
        let mut score = 1;

        let mut count = 0;
        for xx in rev_range_inc(x - 1..=0) {
            count += 1;
            if self.get((xx, y)) >= tree_height {
                break;
            }
        }
        score *= count;

        count = 0;
        for xx in x + 1..self.size_x {
            count += 1;
            if self.get((xx, y)) >= tree_height {
                break;
            }
        }
        score *= count;

        count = 0;
        for yy in rev_range_inc(y - 1..=0) {
            count += 1;
            if self.get((x, yy)) >= tree_height {
                break;
            }
        }
        score *= count;

        count = 0;
        for yy in y + 1..self.size_y {
            count += 1;
            if self.get((x, yy)) >= tree_height {
                break;
            }
        }
        score *= count;

        score
    }

    fn count_highest_scenic_score(&self) -> i32 {
        let mut highest = 0;

        for x in 0..self.size_x {
            for y in 0..self.size_y {
                let score = self.get_scenic_score((x, y));
                if score > highest {
                    highest = score;
                }
            }
        }

        highest
    }

    fn get(&self, (x, y): (u32, u32)) -> u32 {
        *self.map.get(&(x, y)).unwrap_or(&0)
    }
}

// Creates an inclusive range that can be iterated backwards
// e.g. rev_range_inc(5..=2).map(|x| x).collect::<Vec<u32>>()
// results in vec![5, 4, 3, 2]
fn rev_range_inc(range: RangeInclusive<u32>) -> Rev<RangeInclusive<u32>> {
    (*range.end()..=*range.start()).rev()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let tree_map = TreeMap::from_str(&input);
    println!("{}", tree_map.count_highest_scenic_score());
}
