use std::collections::HashSet;
use std::fs;

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
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    tail_visited.insert(tail);

    for movement in movements {
        head = match (movement, head) {
            ('R', (x, y)) => (x + 1, y),
            ('L', (x, y)) => (x - 1, y),
            ('U', (x, y)) => (x, y - 1),
            ('D', (x, y)) => (x, y + 1),
            _ => panic!("Invalid movement"),
        };

        tail = move_tail(head, tail);
        tail_visited.insert(tail);
    }

    tail_visited.len()
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
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
