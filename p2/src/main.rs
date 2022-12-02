use std::{fs::read_to_string, vec};

#[derive(Copy, Clone)]
enum Decision {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

const rock_score: u32 = 1;
const paper_score: u32 = 2;
const scissors_score: u32 = 3;

const lose_score: u32 = 0;
const draw_score: u32 = 3;
const win_score: u32 = 6;

#[derive(Copy, Clone)]
struct Game {
    enemy_decision: Decision,
    required_outcome: Outcome,
}

impl Game {
    fn score(self) -> u32 {
        match self.enemy_decision {
            Decision::Rock => match self.required_outcome {
                Outcome::Draw => rock_score + draw_score,
                Outcome::Lose => scissors_score + lose_score,
                Outcome::Win => paper_score + win_score,
            },
            Decision::Paper => match self.required_outcome {
                Outcome::Win => scissors_score + win_score,
                Outcome::Draw => paper_score + draw_score,
                Outcome::Lose => rock_score + lose_score,
            },
            Decision::Scissors => match self.required_outcome {
                Outcome::Lose => paper_score + lose_score,
                Outcome::Win => rock_score + win_score,
                Outcome::Draw => scissors_score + draw_score,
            },
        }
    }
}

fn str_to_decision(decision_str: &str) -> Decision {
    match decision_str {
        "A" => Decision::Rock,
        "B" => Decision::Paper,
        "C" => Decision::Scissors,

        _ => unreachable!("No such decision"),
    }
}

fn str_to_outcome(outcome_str: &str) -> Outcome {
    match outcome_str {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => unreachable!("No such decision"),
    }
}

fn parse_file(filename: &str) -> Vec<Game> {
    let input: String = read_to_string(filename).expect("Failed to read filename.");

    input
        .lines()
        .map(|line| Game {
            required_outcome: str_to_outcome(&line[2..3]),
            enemy_decision: str_to_decision(&line[0..1]),
        })
        .collect::<Vec<Game>>()
}

fn main() {
    let games = parse_file("inp.txt");
    let score: u32 = games.iter().map(|game| game.score()).sum();
    println!("{}", score);
}
