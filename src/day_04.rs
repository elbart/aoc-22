use std::time::Instant;

use crate::Timing;

fn is_subset_of(subset: (u32, u32), superset: (u32, u32)) -> bool {
    subset.0 >= superset.0 && subset.1 <= superset.1
}

fn is_overlapping(subset: (u32, u32), superset: (u32, u32)) -> bool {
    !(subset.1 < superset.0 || superset.1 < subset.0)
}

pub fn run() -> std::io::Result<Timing> {
    let buffer = include_str!("../input/04/input.txt");

    let start = Instant::now();
    let subsets: u32 = buffer
        .split("\n")
        .map(|line| {
            let (left_range, right_range) = line.split_once(",").unwrap();
            let left_range = left_range.split_once("-").unwrap();
            let left_range: (u32, u32) =
                (left_range.0.parse().unwrap(), left_range.1.parse().unwrap());

            let right_range = right_range.split_once("-").unwrap();
            let right_range: (u32, u32) = (
                right_range.0.parse().unwrap(),
                right_range.1.parse().unwrap(),
            );

            if is_subset_of(left_range, right_range) || is_subset_of(right_range, left_range) {
                return 1;
            }

            0
        })
        .sum();
    let elapsed_part1 = start.elapsed();
    println!("Number of subsets: {}", subsets);

    let start = Instant::now();
    let overlaps: u32 = buffer
        .split("\n")
        .map(|line| {
            let (left_range, right_range) = line.split_once(",").unwrap();
            let left_range = left_range.split_once("-").unwrap();
            let left_range: (u32, u32) =
                (left_range.0.parse().unwrap(), left_range.1.parse().unwrap());

            let right_range = right_range.split_once("-").unwrap();
            let right_range: (u32, u32) = (
                right_range.0.parse().unwrap(),
                right_range.1.parse().unwrap(),
            );

            if is_overlapping(left_range, right_range) {
                return 1;
            }

            0
        })
        .sum();
    let elapsed_part2 = start.elapsed();
    println!("Number of overlaps: {}", overlaps);

    Ok(Timing(elapsed_part1, elapsed_part2))
}

#[cfg(test)]
mod tests {
    use crate::day_04::is_overlapping;

    use super::is_subset_of;

    #[test]
    fn test_is_subset_of() {
        let cases = [
            ((2, 4), (6, 8), false),
            ((6, 8), (2, 4), false),
            ((2, 6), (4, 8), false),
            ((4, 8), (2, 6), false),
            ((2, 6), (2, 8), true),
            ((2, 8), (2, 6), false),
            ((6, 6), (2, 6), true),
            ((2, 6), (6, 6), false),
        ];

        for case in cases {
            assert_eq!(case.2, is_subset_of(case.0, case.1));
        }
    }

    #[test]
    fn test_is_overlapping() {
        let cases = [
            ((2, 4), (6, 8), false),
            ((6, 8), (2, 4), false),
            ((2, 6), (4, 8), true),
            ((4, 8), (2, 6), true),
            ((2, 6), (2, 8), true),
            ((2, 8), (2, 6), true),
            ((6, 6), (2, 6), true),
            ((2, 6), (6, 6), true),
        ];

        for case in cases {
            assert_eq!(case.2, is_overlapping(case.0, case.1));
        }
    }
}
