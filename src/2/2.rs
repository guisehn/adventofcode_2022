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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Round {
    me: Shape,
    opponent: Shape,
    outcome: Outcome,
}

#[derive(Debug, Clone, Copy)]
struct PartialRound {
    opponent: Shape,
    outcome: Outcome,
}

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
        .map(|line| {
            let chars: Vec<char> = line
                .split(" ")
                .flat_map(|shape| shape.to_string().chars().collect::<Vec<char>>())
                .collect();

            let round_goal = PartialRound {
                opponent: char_to_shape(*chars.get(0).expect("Missing 1st char")),
                outcome: char_to_outcome(*chars.get(1).expect("Missing 2nd char")),
            };

            complete_round(round_goal)
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

fn complete_round(partial: PartialRound) -> Round {
    let PartialRound { opponent, outcome } = partial;

    let me = match (opponent, outcome) {
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Rock, Outcome::Lose) => Shape::Scissors,
        (Shape::Rock, Outcome::Draw) => Shape::Rock,

        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Paper, Outcome::Lose) => Shape::Rock,
        (Shape::Paper, Outcome::Draw) => Shape::Paper,

        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Scissors, Outcome::Lose) => Shape::Paper,
        (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
    };

    Round {
        opponent,
        me,
        outcome,
    }
}

fn get_score(round: &Round) -> u32 {
    let Round { me, outcome, .. } = round;

    let shape_score = match me {
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
