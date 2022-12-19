use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;

const DEBUG: bool = false;

type Point = (i32, i32);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Directions {
    items: VecDeque<Direction>,
    current: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Shape {
    points: Vec<Point>,
}

#[derive(Debug)]
struct ShapeFactory {
    shapes: Vec<Shape>,
    current: usize,
}

#[derive(Hash, Debug, Eq, PartialEq)]
struct Seen {
    direction_index: usize,
    shape_index: usize,
    row: String,
}

#[derive(Debug)]
struct SeenWhen {
    round_number: u32,
    height: i32,
}

#[derive(Debug)]
struct Cycle {
    height: u64,
    starts_with_height: u64,
    starts_on_round: u32,
    repeats_every: u64,
    direction_index: usize,
    shape_index: usize,
}

#[derive(Debug)]
struct Tetris {
    falling_shape: Option<Shape>,
    resting_points: HashSet<Point>,
    seen: HashMap<Seen, SeenWhen>,
    cycle_found: Option<Cycle>,
    round_number: u32,
    max_x: i32,
    max_y: i32,
}

impl Direction {
    fn from_char(value: char) -> Direction {
        match value {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Invalid direction {}", value),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

impl Directions {
    fn from_str(value: &str) -> Directions {
        let items = value
            .trim()
            .chars()
            .map(|c| Direction::from_char(c))
            .collect();

        Directions { items, current: 0 }
    }

    fn next(&mut self) -> Direction {
        let current = self.items[self.current];

        self.current += 1;
        if self.current == self.items.len() {
            self.current = 0;
        }

        current
    }
}

impl Shape {
    fn new() -> Shape {
        Shape { points: vec![] }
    }

    fn move_offset(&mut self, offset_x: i32, offset_y: i32) {
        for i in 0..self.points.len() {
            let (x, y) = self.points[i];
            self.points[i] = (x + offset_x, y + offset_y);
        }
    }

    fn move_to(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.move_x(-1),
            Direction::Right => self.move_x(1),
        }
    }

    fn move_x(&mut self, offset: i32) {
        self.move_offset(offset, 0);
    }

    fn move_y(&mut self, offset: i32) {
        self.move_offset(0, offset);
    }
}

impl ShapeFactory {
    fn from_str(input: &str) -> ShapeFactory {
        let shapes_str = input.split("\n\n");
        let mut shapes: Vec<Shape> = vec![];

        for shape in shapes_str {
            let lines: Vec<String> = shape.lines().map(|v| v.to_string()).collect();
            let height = lines.len();
            let mut shape = Shape::new();

            for y in 0..height {
                let line: Vec<char> = lines[y].chars().collect();

                for x in 0..line.len() {
                    if line[x] == '#' {
                        shape.points.push((x as i32, (height - y - 1) as i32));
                    }
                }
            }

            shapes.push(shape);
        }

        ShapeFactory { shapes, current: 0 }
    }

    fn produce(&mut self) -> Shape {
        let current = &self.shapes[self.current];

        self.current += 1;
        if self.current == self.shapes.len() {
            self.current = 0;
        }

        current.clone()
    }

    fn produce_at(&mut self, (x, y): Point) -> Shape {
        let mut shape = self.produce();
        shape.move_x(x);
        shape.move_y(y);
        shape
    }
}

impl Tetris {
    fn new() -> Tetris {
        Tetris {
            max_x: 6,
            max_y: 0,
            resting_points: HashSet::new(),
            seen: HashMap::new(),
            falling_shape: None,
            round_number: 0,
            cycle_found: None,
        }
    }

    fn move_falling_shape_to(&mut self, direction: Direction) -> bool {
        if self.falling_shape.is_none() {
            return false;
        }

        let falling_shape = self.falling_shape.as_ref().unwrap();
        let mut moved_shape = falling_shape.clone();
        moved_shape.move_to(direction);

        let has_overflown = moved_shape
            .points
            .iter()
            .any(|&(x, y)| x < 0 || x > self.max_x || self.resting_points.contains(&(x, y)));
        if has_overflown {
            return false;
        }

        self.falling_shape = Some(moved_shape);
        true
    }

    fn move_falling_shape_down(&mut self) -> bool {
        if self.falling_shape.is_none() {
            return false;
        }

        let falling_shape = self.falling_shape.as_ref().unwrap();
        let mut moved_shape = falling_shape.clone();
        moved_shape.move_y(-1);

        let has_overflown = moved_shape
            .points
            .iter()
            .any(|&(x, y)| self.resting_points.contains(&(x, y)) || y < 0);
        if has_overflown {
            return false;
        }

        self.falling_shape = Some(moved_shape);
        true
    }

    fn round(&mut self, shape_factory: &mut ShapeFactory, movements: &mut Directions) {
        self.round_number += 1;
        self.falling_shape = Some(shape_factory.produce_at((2, self.max_y + 3)));

        if DEBUG {
            println!("A new rock begins falling:");
            self.print();
        }

        loop {
            let direction = movements.next();
            if DEBUG {
                println!("Jet of gas attempts to push rock {}", direction);
            }
            self.move_falling_shape_to(direction);
            if DEBUG {
                self.print();
            }

            if self.move_falling_shape_down() {
                if DEBUG {
                    println!("Rock falls 1 unit");
                    self.print();
                }
            } else {
                if DEBUG {
                    println!("Rock comes to rest");
                }
                self.move_falling_shape_to_resting_points();
                if DEBUG {
                    self.print();
                }

                break;
            }
        }

        if self.cycle_found.is_none() {
            let row = self.get_row(self.max_y - 1);

            if !row.eq("#######") {
                return;
            }

            let shape_index = shape_factory.current;
            let direction_index = movements.current;
            let seen = Seen {
                row,
                shape_index,
                direction_index,
            };

            if self.seen.contains_key(&seen) {
                println!("FOUND!!!!");
                // dbg!(&seen);
                let when = self.seen.get(&seen).unwrap();
                dbg!(&seen);
                dbg!(&self.round_number);
                dbg!(&when);
                // self.print();
                // self.cycle_found = true;

                self.cycle_found = Some(Cycle {
                    height: (self.max_y - when.height) as u64,
                    starts_with_height: when.height as u64,
                    starts_on_round: when.round_number,
                    repeats_every: (self.round_number - when.round_number) as u64,
                    direction_index: direction_index,
                    shape_index: shape_index,
                });
            } else {
                self.seen.insert(
                    seen,
                    SeenWhen {
                        round_number: self.round_number,
                        height: self.max_y,
                    },
                );
            }
        }
    }

    fn move_falling_shape_to_resting_points(&mut self) -> bool {
        if self.falling_shape.is_none() {
            return false;
        }

        let falling_shape = self.falling_shape.as_ref().unwrap();
        for point in &falling_shape.points {
            self.resting_points.insert(*point);
        }

        let max_y = falling_shape
            .points
            .iter()
            .map(|&(_x, y)| y)
            .max()
            .unwrap_or(0)
            + 1;
        if max_y > self.max_y {
            self.max_y = max_y;
        }

        self.falling_shape = None;
        true
    }

    fn get_row(&self, y: i32) -> String {
        let empty_shape = Shape::new();
        let falling_shape = self.falling_shape.as_ref().unwrap_or(&empty_shape);
        let mut result: Vec<char> = vec![];

        for x in 0..=self.max_x {
            let point = (x, y);

            let piece = if falling_shape.points.contains(&point) {
                '@'
            } else if self.resting_points.contains(&point) {
                '#'
            } else {
                '.'
            };

            result.push(piece);
        }

        result.iter().collect()
    }

    fn print(&self) {
        let y_margin = 5;

        for reverse_y in 0..=(self.max_y + y_margin) {
            let y = (self.max_y + y_margin) - reverse_y;

            print!("{:01$} ", y, 6);
            print!("|{}|", self.get_row(y));
            println!("");
        }

        print!("       +");
        for _ in 0..=self.max_x {
            print!("-");
        }
        print!("+\n\n");
    }
}

fn main() {
    let shapes_input = fs::read_to_string("shapes.txt").unwrap();
    let mut shape_factory = ShapeFactory::from_str(&shapes_input);

    let directions_input = fs::read_to_string("input.txt").unwrap();
    let mut directions = Directions::from_str(&directions_input);

    let mut tetris = Tetris::new();
    while tetris.cycle_found.is_none() {
        tetris.round(&mut shape_factory, &mut directions);
    }

    println!("Final tower:");
    // tetris.print();

    let cycle = tetris.cycle_found.unwrap();

    dbg!(&cycle);

    let asked_round: u64 = 1000000000000;

    // println!("Height: {}", tetris.max_y);

    println!("trying again...");

    // // for _ in 0..16 + 35 + 35 {
    // //     new_tetris.round(&mut shape_factory, &mut directions);
    // // }
    // for _ in 0..561 + 1725 + 1725 {
    //     new_tetris.round(&mut shape_factory, &mut directions);
    // }
    // dbg!(&shape_factory.current);
    // dbg!(&directions.current);
    // new_tetris.print();

    let mut height: u64 = cycle.starts_with_height;
    let mut round = cycle.starts_on_round as u64;

    loop {
        if round + cycle.repeats_every > asked_round {
            break;
        }
        height += cycle.height;
        round += cycle.repeats_every;
    }

    let mut new_tetris = Tetris::new();
    shape_factory.current = cycle.shape_index;
    directions.current = cycle.direction_index;
    for _ in 0..asked_round - round {
        new_tetris.round(&mut shape_factory, &mut directions);
    }

    let result = height + new_tetris.max_y as u64;
    println!("{}", result);
}
