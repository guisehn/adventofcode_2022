use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point(i32, i32, i32); // x, y, z

#[derive(Debug, Eq, PartialEq)]
enum Object {
    Lava,
    Water,
}

#[derive(Debug)]
struct Problem {
    items: HashMap<Point, Object>,
    max_x: i32,
    max_y: i32,
    max_z: i32,
}

impl Problem {
    fn new(lava_points: Vec<Point>) -> Problem {
        let mut items = HashMap::new();
        let (mut max_x, mut max_y, mut max_z) = (0, 0, 0);
        for lava_point in lava_points {
            let Point(x, y, z) = lava_point;
            items.insert(lava_point, Object::Lava);
            max_x = cmp::max(x, max_x);
            max_y = cmp::max(y, max_y);
            max_z = cmp::max(z, max_z);
        }
        Problem {
            items,
            max_x,
            max_y,
            max_z,
        }
    }

    fn water_fill(&mut self) {
        let mut queue = VecDeque::from([Point(0, 0, 0)]);
        let mut queue_items = HashSet::<Point>::new(); // for O(1) duplicate lookup

        while let Some(point) = queue.pop_front() {
            queue_items.remove(&point);
            self.items.insert(point, Object::Water);

            for adj in point.adjacent_points() {
                if self.items.contains_key(&adj)
                    || !self.is_valid_point(adj)
                    || queue_items.contains(&adj)
                {
                    continue;
                }

                queue.push_back(adj);
                queue_items.insert(adj);
            }
        }
    }

    fn total_lava_surface_area(&self) -> usize {
        self.items
            .iter()
            .map(|(point, _)| self.lava_surface_area(point))
            .sum()
    }

    fn lava_surface_area(&self, point: &Point) -> usize {
        if !self.has_lava_at(point) {
            return 0;
        }

        point
            .adjacent_points()
            .iter()
            .filter(|adj_point| self.has_water_at(adj_point))
            .count()
    }

    fn has_lava_at(&self, point: &Point) -> bool {
        if !self.items.contains_key(point) {
            return false;
        }

        *self.items.get(point).unwrap() == Object::Lava
    }

    fn has_water_at(&self, point: &Point) -> bool {
        if !self.items.contains_key(point) {
            return false;
        }

        *self.items.get(point).unwrap() == Object::Water
    }

    fn is_valid_point(&self, Point(x, y, z): Point) -> bool {
        x >= -1
            && y >= -1
            && z >= -1
            && x <= self.max_x + 1
            && y <= self.max_y + 1
            && z <= self.max_z + 1
    }
}

impl Point {
    fn from_str(input: &str) -> Point {
        let values: Vec<i32> = input.split(",").map(|val| val.parse().unwrap()).collect();
        Point(values[0], values[1], values[2])
    }

    fn adjacent_points(&self) -> [Point; 6] {
        let Point(x, y, z) = *self;

        [
            Point(x, y, z - 1),
            Point(x, y, z + 1),
            Point(x, y - 1, z),
            Point(x, y + 1, z),
            Point(x - 1, y, z),
            Point(x + 1, y, z),
        ]
    }
}

fn main() {
    let points = get_input_points();
    let mut problem = Problem::new(points);
    problem.water_fill();
    let result = problem.total_lava_surface_area();
    println!("{result}");
}

fn get_input_points() -> Vec<Point> {
    let input = fs::read_to_string("input.txt").unwrap();

    input
        .trim()
        .lines()
        .map(|line| Point::from_str(line))
        .collect()
}
