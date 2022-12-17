use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::fs;

const DEBUG: bool = false;

type Point = (i32, i32);

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Directions {
    items: VecDeque<Direction>,
    current: usize,
}

#[derive(Debug, Clone)]
struct Shape {
    points: Vec<Point>,
}

#[derive(Debug)]
struct ShapeFactory {
    shapes: Vec<Shape>,
    current: usize,
}

#[derive(Debug)]
struct Tetris {
    falling_shape: Option<Shape>,
    resting_points: HashSet<Point>,
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
            falling_shape: None,
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

    fn print(&self) {
        let empty_shape = Shape::new();
        let falling_shape = self.falling_shape.as_ref().unwrap_or(&empty_shape);
        let y_margin = 5;

        for reverse_y in 0..=(self.max_y + y_margin) {
            let y = (self.max_y + y_margin) - reverse_y;

            print!("|");

            for x in 0..=self.max_x {
                let point = (x, y);

                if falling_shape.points.contains(&point) {
                    print!("@");
                } else if self.resting_points.contains(&point) {
                    print!("#");
                } else {
                    print!(".");
                }
            }

            print!("|");
            println!("");
        }

        print!("+");
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
    for _ in 0..2022 {
        tetris.round(&mut shape_factory, &mut directions);
    }

    println!("Final tower:");
    tetris.print();

    println!("Height: {}", tetris.max_y);
}
