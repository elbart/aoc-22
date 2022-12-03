use std::{collections::HashSet, time::Instant};

use crate::Timing;

fn find_shared_item(line: &str) -> u8 {
    let parts = line.split_at(&line.len() / 2);
    let unique_left: HashSet<u8> = HashSet::from_iter(parts.0.as_bytes().iter().cloned());
    let unique_right: HashSet<u8> = HashSet::from_iter(parts.1.as_bytes().iter().cloned());
    let mut intersection = unique_left.intersection(&unique_right);
    let duplicate = intersection.next().unwrap();
    assert_eq!(None, intersection.next());

    if duplicate < &97 {
        duplicate - 38
    } else {
        duplicate - 96
    }
}

fn find_shared_item_3lines(lines: &[&str; 3]) -> u8 {
    let unique_left: HashSet<u8> = HashSet::from_iter(lines[0].as_bytes().iter().cloned());
    let unique_mid: HashSet<u8> = HashSet::from_iter(lines[1].as_bytes().iter().cloned());
    let unique_right = HashSet::from_iter(lines[2].as_bytes().iter().cloned());
    let first_intersection =
        HashSet::<u8>::from_iter(unique_left.intersection(&unique_mid).cloned());
    let mut final_intersection = first_intersection.intersection(&unique_right);
    let duplicate = final_intersection.next().unwrap();
    assert_eq!(None, final_intersection.next());

    if duplicate < &97 {
        duplicate - 38
    } else {
        duplicate - 96
    }
}

pub fn run() -> std::io::Result<Timing> {
    let buffer = include_str!("../input/03/input.txt");

    let start = Instant::now();
    let sum_priority: u32 = buffer
        .split("\n")
        .map(|raw| find_shared_item(raw) as u32)
        .sum();
    let elapsed_part1 = start.elapsed();
    println!("Sum of priorities: {}", sum_priority);

    let start = Instant::now();
    let mut sum_3lines = 0;
    let mut shared: [&str; 3] = ["", "", ""];
    for (idx, line) in buffer.split("\n").enumerate() {
        if idx > 0 && idx % 3 == 0 {
            sum_3lines += find_shared_item_3lines(&shared) as u32;
            shared = ["", "", ""];
        }
        shared[idx % 3] = line;
    }
    sum_3lines += find_shared_item_3lines(&shared) as u32;
    let elapsed_part2 = start.elapsed();
    println!("Sum of priorities: {}", sum_3lines);

    Ok(Timing(elapsed_part1, elapsed_part2))
}
