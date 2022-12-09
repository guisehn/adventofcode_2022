use std::collections::HashMap;
use std::fs;
use std::ops::Range;
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

        let directions: Vec<Vec<(u32, u32)>> = vec![
            gen_positions_reverse(y - 1..=0, |y| (x, y)),  // up
            gen_positions(y + 1..self.size_y, |y| (x, y)), // down
            gen_positions_reverse(x - 1..=0, |x| (x, y)),  // left
            gen_positions(x + 1..self.size_x, |x| (x, y)), // right
        ];

        let mut score = 1;
        let tree_height = self.get((x, y));

        for direction in directions {
            let mut count = 0;

            for pos in direction {
                count += 1;
                if self.get(pos) >= tree_height {
                    break;
                }
            }

            score *= count;
        }

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

fn gen_positions_reverse<F: Fn(u32) -> (u32, u32)>(
    range: RangeInclusive<u32>,
    fun: F,
) -> Vec<(u32, u32)> {
    (*range.end()..=*range.start()).rev().map(fun).collect()
}

fn gen_positions<F: Fn(u32) -> (u32, u32)>(range: Range<u32>, fun: F) -> Vec<(u32, u32)> {
    range.map(fun).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let tree_map = TreeMap::from_str(&input);
    println!("{}", tree_map.count_highest_scenic_score());
}
