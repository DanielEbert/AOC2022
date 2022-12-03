use std::collections::BTreeSet;
use std::fs;

fn item_prio(item: char) -> u32 {
    if 'a' <= item && item <= 'z' {
        return item as u32 - 'a' as u32 + 1;
    } else if 'A' <= item && item <= 'Z' {
        return item as u32 - 'A' as u32 + 27;
    }
    panic!("unknown item");
}

fn main() {
    let input = fs::read_to_string("inp.txt").expect("Failed to read input.");
    let mut total_pio: u32 = 0;

    for line in input.lines() {
        let (compartment1, compartment2) = line.split_at(line.len() / 2);
        let compartment1 = BTreeSet::from_iter(compartment1.chars());
        let compartment2 = BTreeSet::from_iter(compartment2.chars());
        let shared = *compartment1
            .intersection(&compartment2)
            .next()
            .expect("No same item found");
        println!("diff {}", shared);
        total_pio += item_prio(shared);
    }

    println!("Total Pio: {}", total_pio);
}
