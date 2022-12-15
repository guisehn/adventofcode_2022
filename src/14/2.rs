use std::cmp;
use std::collections::HashMap;
use std::fs;

const DRAW_WHILE_FALLING: bool = false;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Point(u32, u32);

#[derive(Debug)]
enum Object {
    Rock,
    Sand,
}

#[derive(Debug)]
struct Cave {
    map: HashMap<Point, Object>,
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
}

impl Point {
    fn path_to(&self, other: &Point) -> Vec<Point> {
        let Point(x, y) = self;
        let Point(other_x, other_y) = other;

        if x == other_x {
            let (min_y, max_y) = (cmp::min(y, other_y), cmp::max(y, other_y));
            (*min_y..=*max_y)
                .into_iter()
                .map(|y| Point(*x, y))
                .collect()
        } else if y == other_y {
            let (min_x, max_x) = (cmp::min(x, other_x), cmp::max(x, other_x));
            (*min_x..=*max_x)
                .into_iter()
                .map(|x| Point(x, *y))
                .collect()
        } else {
            todo!()
        }
    }
}

impl Object {
    fn to_char(&self) -> char {
        match self {
            Object::Rock => '#',
            Object::Sand => 'o',
        }
    }
}

impl Cave {
    fn new() -> Cave {
        Cave {
            map: HashMap::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    fn add_rocks_from(&mut self, input: &str) {
        for path in input.trim().lines() {
            let points: Vec<Point> = path
                .trim()
                .split(" -> ")
                .map(|point| point.split_once(',').unwrap())
                .map(|(x, y)| Point(x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
                .collect();

            for i in 1..points.len() {
                let path = points[i].path_to(&points[i - 1]);
                for point in path {
                    self.map.insert(point, Object::Rock);
                    self.update_boundaries(&point);
                }
            }
        }
    }

    fn update_boundaries(&mut self, point: &Point) {
        let Point(x, y) = point;

        if x > &self.max_x || self.max_x == 0 {
            self.max_x = *x;
        }
        if x < &self.min_x || self.min_x == 0 {
            self.min_x = *x;
        }
        if y > &self.max_y || self.max_y == 0 {
            self.max_y = *y;
        }
        if y < &self.min_y || self.min_y == 0 {
            self.min_y = *y;
        }
    }

    fn drop_sand(&mut self, origin: Point) -> bool {
        let mut sand = origin;

        self.map.insert(sand, Object::Sand);

        if DRAW_WHILE_FALLING {
            self.draw();
        }

        loop {
            let Point(mut x, mut y) = sand;

            if !self.has_object(&Point(x, y + 1)) {
                y += 1;
            } else {
                if !self.has_object(&Point(x - 1, y + 1)) {
                    y += 1;
                    x -= 1;
                } else if !self.has_object(&Point(x + 1, y + 1)) {
                    y += 1;
                    x += 1;
                } else {
                    break;
                }
            }

            self.map.remove(&sand);
            sand = Point(x, y);
            self.map.insert(sand, Object::Sand);

            if DRAW_WHILE_FALLING {
                self.draw();
                println!("");
            }
        }

        sand != origin
    }

    fn has_object(&self, point: &Point) -> bool {
        self.map.contains_key(point) || self.touches_floor(point)
    }

    fn touches_floor(&self, point: &Point) -> bool {
        let Point(_x, y) = point;
        let floor_y = self.max_y + 2;
        *y == floor_y
    }

    fn draw(&self) {
        let range_x = (self.min_x as i32 - 10)..(self.max_x as i32 + 10);
        let range_y = (self.min_y as i32 - 3)..(self.max_y as i32 + 3);

        for y in range_y {
            for x in range_x.clone() {
                let point = Point(x as u32, y as u32);

                let char = if self.touches_floor(&point) {
                    '='
                } else {
                    match self.map.get(&point) {
                        Some(obj) => obj.to_char(),
                        None => '.',
                    }
                };

                print!("{char}");
            }

            println!("");
        }
    }
}

fn main() {
    let input = read_input();

    let mut cave = Cave::new();
    cave.add_rocks_from(&input);

    let mut units = 0;
    while cave.drop_sand(Point(500, 0)) {
        units += 1;
    }

    cave.draw();

    println!("\nResult: {}", units + 1);
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}
