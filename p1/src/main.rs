use std::fs::read_to_string;
use std::vec::Vec;

#[derive(Default)]
struct Elf {
    calories: u32,
}

fn main() {
    let input = read_to_string("inp.txt").expect("Failed to read inp.txt.");
    let mut elfes: Vec<Elf> = vec![];

    for calorie_group in input.split("\n\n") {
        elfes.push(Elf {
            calories: calorie_group
                .lines()
                .map(|line| {
                    line.parse::<u32>()
                        .expect("Failed to parse line from str to int.")
                })
                .sum(),
        });
    }

    let mut elf_calories = elfes
        .iter()
        .map(|e| -> u32 { e.calories })
        .collect::<Vec<u32>>();

    elf_calories.sort_by(|a, b| b.cmp(a));
    let max_3_calories: u32 = elf_calories.iter().take(3).sum();

    println!("{}", max_3_calories);
}
