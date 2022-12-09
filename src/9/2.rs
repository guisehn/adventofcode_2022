use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
}

impl Rope {
    fn new(size: usize) -> Rope {
        Rope {
            knots: vec![(0, 0); size],
        }
    }

    fn move_rope(&mut self, direction: char) {
        self.knots[0] = Rope::move_knot(*self.knots.get(0).unwrap(), direction);

        for i in 1..self.knots.len() {
            let new_value =
                Rope::follow_knot(*self.knots.get(i - 1).unwrap(), *self.knots.get(i).unwrap());

            self.knots[i] = new_value;
        }
    }

    fn move_knot(knot: (i32, i32), direction: char) -> (i32, i32) {
        match (direction, knot) {
            ('R', (x, y)) => (x + 1, y),
            ('L', (x, y)) => (x - 1, y),
            ('U', (x, y)) => (x, y - 1),
            ('D', (x, y)) => (x, y + 1),
            _ => panic!("Invalid direction"),
        }
    }

    fn follow_knot(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
        let (head_x, head_y) = head;
        let (mut tail_x, mut tail_y) = tail;

        if head_x == tail_x {
            if head_y - tail_y > 1 {
                tail_y += 1;
            } else if tail_y - head_y > 1 {
                tail_y -= 1;
            }
        } else if head_y == tail_y {
            if head_x - tail_x > 1 {
                tail_x += 1;
            } else if tail_x - head_x > 1 {
                tail_x -= 1;
            }
        } else if ((head_x - tail_x).abs() > 1) || ((head_y - tail_y).abs() > 1) {
            if head_x > tail_x {
                tail_x += 1;
            } else {
                tail_x -= 1;
            }

            if head_y > tail_y {
                tail_y += 1;
            } else {
                tail_y -= 1;
            }
        }

        (tail_x, tail_y)
    }

    fn tail(&self) -> &(i32, i32) {
        self.knots.last().unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let movements = parse_movements(input);
    let solution = solve(movements);
    println!("{solution}");
}

fn parse_movements(input: String) -> Vec<char> {
    let mut movements = vec![];

    for line in input.trim().lines() {
        let (direction_str, quantity_str) = line.split_once(' ').unwrap();
        let quantity: i32 = quantity_str.parse().unwrap();
        let direction = direction_str.chars().next().unwrap();

        for _ in 0..quantity {
            movements.push(direction);
        }
    }

    movements
}

fn solve(movements: Vec<char>) -> usize {
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Rope = Rope::new(10);

    tail_visited.insert(*rope.tail());

    for direction in movements {
        rope.move_rope(direction);
        tail_visited.insert(*rope.tail());
    }

    tail_visited.len()
}
