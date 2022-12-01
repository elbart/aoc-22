use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::time::Instant;

use crate::Timing;

pub fn run() -> io::Result<Timing> {
    let f = File::open("./input/01/input.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    // split by double newline first, to create an iterator over each group of calories
    let start = Instant::now();
    let elves_accumulated = buffer.split("\n\n").map(|calorie_list| {
        calorie_list
            .split("\n")
            .map(|calorie_item| calorie_item.parse::<u32>().unwrap())
            .sum()
    });
    let elapsed_part1 = start.elapsed();

    let top_elf = elves_accumulated.clone().max().unwrap();
    println!("top elf calories: {}", top_elf);

    // top 3 elves need a sorted structure -> thanks we have a BinaryHeap for that
    let start = Instant::now();
    let top_3_elves: u32 = elves_accumulated
        .collect::<BinaryHeap<u32>>()
        .iter()
        .take(3)
        .sum();
    let elapsed_part2 = start.elapsed();

    println!("top 3 elf calories: {}", top_3_elves);

    Ok(Timing(elapsed_part1, elapsed_part2))
}
