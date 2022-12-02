use std::fs;

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

#[derive(Debug, Clone, Copy)]
struct Round(Shape, Shape, Outcome);

#[derive(Debug, Clone, Copy)]
struct RoundGoal(Shape, Outcome);

fn main() {
    let input = read_input();
    let rounds = get_rounds(input);
    let scores: Vec<u32> = rounds.iter().map(|round| get_score(round)).collect();
    let total: u32 = scores.iter().sum();
    println!("{total}");
}

fn read_input() -> String {
    fs::read_to_string("input.txt").expect("Error reading file")
}

fn get_rounds(input: String) -> Vec<Round> {
    input
        .split("\n")
        .map(|line| line.split(" "))
        .map(|line| {
            let chars: Vec<char> = line
                .flat_map(|shape| shape.to_string().chars().collect::<Vec<char>>())
                .collect();

            let round_goal = RoundGoal(
                char_to_shape(*chars.get(0).expect("Missing 1st char")),
                char_to_outcome(*chars.get(1).expect("Missing 2nd char")),
            );

            to_round(round_goal)
        })
        .collect()
}

fn char_to_shape(c: char) -> Shape {
    match c {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("Unknown char '{}'", c),
    }
}

fn char_to_outcome(c: char) -> Outcome {
    match c {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("Unknown char '{}'", c),
    }
}

fn get_score(round: &Round) -> u32 {
    let Round(_opponent_shape, my_shape, outcome) = round;

    let shape_score = match my_shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    let outcome_score = match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    };

    shape_score + outcome_score
}

fn to_round(round_goal: RoundGoal) -> Round {
    let RoundGoal(opponent_shape, outcome) = round_goal;

    let my_shape = match round_goal {
        RoundGoal(Shape::Rock, Outcome::Win) => Shape::Paper,
        RoundGoal(Shape::Rock, Outcome::Lose) => Shape::Scissors,
        RoundGoal(Shape::Rock, Outcome::Draw) => Shape::Rock,

        RoundGoal(Shape::Paper, Outcome::Win) => Shape::Scissors,
        RoundGoal(Shape::Paper, Outcome::Lose) => Shape::Rock,
        RoundGoal(Shape::Paper, Outcome::Draw) => Shape::Paper,

        RoundGoal(Shape::Scissors, Outcome::Win) => Shape::Rock,
        RoundGoal(Shape::Scissors, Outcome::Lose) => Shape::Paper,
        RoundGoal(Shape::Scissors, Outcome::Draw) => Shape::Scissors,
    };

    Round(opponent_shape, my_shape, outcome)
}
