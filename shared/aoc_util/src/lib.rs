use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|l| l.unwrap())
}

pub fn read_ints<P>(filename: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let file_content = std::fs::read_to_string(filename).expect("Failed to read file");
    file_content
        .split(",")
        .map(|i| FromStr::from_str(i).unwrap())
        .collect()
}
