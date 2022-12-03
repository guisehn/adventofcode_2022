use std::fs;

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Won,
    Draw,
    Lost,
}

#[derive(Debug)]
struct Round(Shape, Shape);

fn main() {
    let input = read_input();
    let rounds = get_rounds(input);
    let scores: Vec<u32> = rounds.iter().map(|round| get_score(round)).collect();
    let total: u32 = scores.iter().sum();
    println!("{total}");
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn get_rounds(input: String) -> Vec<Round> {
    input
        .split("\n")
        .map(|line| line.split(" "))
        .map(|line| {
            let chars: Vec<char> = line
                .flat_map(|shape| shape.to_string().chars().collect::<Vec<char>>())
                .collect();

            Round(
                char_to_shape(*chars.get(0).unwrap()),
                char_to_shape(*chars.get(1).unwrap()),
            )
        })
        .collect()
}

fn char_to_shape(c: char) -> Shape {
    match c {
        'A' => Shape::Rock,
        'X' => Shape::Rock,
        'B' => Shape::Paper,
        'Y' => Shape::Paper,
        'C' => Shape::Scissors,
        'Z' => Shape::Scissors,
        _ => panic!("Unknown char '{}'", c),
    }
}

fn get_score(round: &Round) -> u32 {
    let Round(_opponent_shape, my_shape) = &round;

    let shape_score = match my_shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    let outcome_score = match get_outcome(round) {
        Outcome::Lost => 0,
        Outcome::Draw => 3,
        Outcome::Won => 6,
    };

    shape_score + outcome_score
}

fn get_outcome(round: &Round) -> Outcome {
    match round {
        Round(Shape::Rock, Shape::Rock) => Outcome::Draw,
        Round(Shape::Rock, Shape::Paper) => Outcome::Won,
        Round(Shape::Rock, Shape::Scissors) => Outcome::Lost,

        Round(Shape::Paper, Shape::Rock) => Outcome::Lost,
        Round(Shape::Paper, Shape::Paper) => Outcome::Draw,
        Round(Shape::Paper, Shape::Scissors) => Outcome::Won,

        Round(Shape::Scissors, Shape::Rock) => Outcome::Won,
        Round(Shape::Scissors, Shape::Paper) => Outcome::Lost,
        Round(Shape::Scissors, Shape::Scissors) => Outcome::Draw,
    }
}
