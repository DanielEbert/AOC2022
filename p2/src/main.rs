use std::{fs::read_to_string, vec};

#[derive(Copy, Clone)]
enum Decision {
    Rock,
    Paper,
    Scissors,
}

const rock_score: u32 = 1;
const paper_score: u32 = 2;
const scissors_score: u32 = 3;

const lose_score: u32 = 0;
const draw_score: u32 = 3;
const win_score: u32 = 6;

#[derive(Copy, Clone)]
struct Game {
    player_decision: Decision,
    enemy_decision: Decision,
}

impl Game {
    fn score(self) -> u32 {
        match self.player_decision {
            Decision::Rock => match self.enemy_decision {
                Decision::Rock => rock_score + draw_score,
                Decision::Paper => rock_score + lose_score,
                Decision::Scissors => rock_score + win_score,
            },
            Decision::Paper => match self.enemy_decision {
                Decision::Rock => paper_score + win_score,
                Decision::Paper => paper_score + draw_score,
                Decision::Scissors => paper_score + lose_score,
            },
            Decision::Scissors => match self.enemy_decision {
                Decision::Rock => scissors_score + lose_score,
                Decision::Paper => scissors_score + win_score,
                Decision::Scissors => scissors_score + draw_score,
            },
        }
    }
}

/// TODO: first column is enemy play, second column is my recommended play

fn str_to_decision(decision_str: &str) -> Decision {
    match decision_str {
        "A" => Decision::Rock,
        "B" => Decision::Paper,
        "C" => Decision::Scissors,
        "X" => Decision::Rock,
        "Y" => Decision::Paper,
        "Z" => Decision::Scissors,
        _ => unreachable!("No such decision"),
    }
}

fn parse_file(filename: &str) -> Vec<Game> {
    let input: String = read_to_string(filename).expect("Failed to read filename.");

    input
        .lines()
        .map(|line| Game {
            player_decision: str_to_decision(&line[2..3]),
            enemy_decision: str_to_decision(&line[0..1]),
        })
        .collect::<Vec<Game>>()
}

fn main() {
    let games = parse_file("inp.txt");
    let score: u32 = games.iter().map(|game| game.score()).sum();
    println!("{}", score);
}
