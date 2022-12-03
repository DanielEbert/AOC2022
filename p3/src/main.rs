use std::collections::BTreeSet;
use std::fs;

fn item_prio(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => panic!("unknown item"),
    }
}

fn main() {
    let input = fs::read_to_string("inp.txt").expect("Failed to read input.");

    let mut total_pio: u32 = 0;

    let lines = input.lines().collect::<Vec<_>>();

    for i in (0..lines.len()).step_by(3) {
        let a = BTreeSet::from_iter(lines[i].chars());
        let b = BTreeSet::from_iter(lines[i + 1].chars());
        let c = BTreeSet::from_iter(lines[i + 2].chars());

        let a_b_shared = BTreeSet::from_iter(a.intersection(&b).into_iter().copied());
        let mut shared = a_b_shared.intersection(&c);

        let shared_item = *shared.next().expect("No shared item.");

        total_pio += item_prio(shared_item);
    }

    println!("Total Pio: {}", total_pio);
}
