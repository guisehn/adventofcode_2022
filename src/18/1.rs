use std::collections::HashSet;
use std::fs;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Cube {
    x: u32,
    y: u32,
    z: u32,
}

impl Cube {
    fn new((x, y, z): (u32, u32, u32)) -> Cube {
        Cube { x, y, z }
    }

    fn from_str(input: &str) -> Cube {
        let values: Vec<u32> = input.split(",").map(|val| val.parse().unwrap()).collect();
        Cube {
            x: values[0],
            y: values[1],
            z: values[2],
        }
    }

    fn surface_area(&self, other_cubes: &HashSet<Cube>) -> u32 {
        let adjacent_cubes = [
            Cube::new((self.x, self.y, self.z - 1)),
            Cube::new((self.x, self.y, self.z + 1)),
            Cube::new((self.x, self.y - 1, self.z)),
            Cube::new((self.x, self.y + 1, self.z)),
            Cube::new((self.x - 1, self.y, self.z)),
            Cube::new((self.x + 1, self.y, self.z)),
        ];

        let existing_adjacent_cubes: usize = adjacent_cubes
            .iter()
            .filter(|adj_cube| other_cubes.contains(adj_cube))
            .count();

        (adjacent_cubes.len() - existing_adjacent_cubes) as u32
    }
}

fn main() {
    let cubes = get_cubes();
    let total_surface_area: u32 = cubes.iter().map(|cube| cube.surface_area(&cubes)).sum();
    println!("{total_surface_area}");
}

fn get_cubes() -> HashSet<Cube> {
    let input = fs::read_to_string("input.txt").unwrap();

    input
        .trim()
        .lines()
        .map(|line| Cube::from_str(line))
        .collect()
}
