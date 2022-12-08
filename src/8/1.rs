use std::collections::HashMap;
use std::fs;

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

    fn count_trees_visible(&self) -> i32 {
        (0..self.size_x)
            .map(|x| {
                (0..self.size_y)
                    .filter(|y| self.is_tree_visible((x, *y)))
                    .count() as i32
            })
            .sum()
    }

    fn is_tree_visible(&self, pos: (u32, u32)) -> bool {
        self.is_tree_visible_horizontally(pos) || self.is_tree_visible_vertically(pos)
    }

    fn is_tree_visible_horizontally(&self, (x, y): (u32, u32)) -> bool {
        if x == 0 || x == self.size_x - 1 {
            return true;
        }

        let tree_height = self.get((x, y));
        let any_taller_before = (0..x).any(|xx| self.get((xx, y)) >= tree_height);
        let any_taller_after = (x + 1..self.size_x).any(|xx| self.get((xx, y)) >= tree_height);
        !(any_taller_before && any_taller_after)
    }

    fn is_tree_visible_vertically(&self, (x, y): (u32, u32)) -> bool {
        if y == 0 || y == self.size_y - 1 {
            return true;
        }

        let tree_height = self.get((x, y));
        let any_taller_before = (0..y).any(|yy| self.get((x, yy)) >= tree_height);
        let any_taller_after = (y + 1..self.size_y).any(|yy| self.get((x, yy)) >= tree_height);
        !(any_taller_before && any_taller_after)
    }

    fn get(&self, (x, y): (u32, u32)) -> u32 {
        *self.map.get(&(x, y)).unwrap_or(&0)
    }
}

fn main() {
    let input = read_input();
    let tree_map = TreeMap::from_str(&input);
    println!("{}", tree_map.count_trees_visible());
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}
