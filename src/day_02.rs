use std::time::Instant;

use crate::Timing;

trait Score {
    fn score(&self) -> u32;
}

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

struct ChoiceChoice(Choice, Choice);

impl ChoiceChoice {
    fn new(pair: (&str, &str)) -> Self {
        Self(pair.0.into(), pair.1.into())
    }
}

struct ChoiceOutcome(Choice, Outcome);

impl ChoiceOutcome {
    fn new(pair: (&str, &str)) -> Self {
        Self(pair.0.into(), pair.1.into())
    }
}

impl Score for ChoiceChoice {
    fn score(&self) -> u32 {
        self.1.score()
            + match (&self.0, &self.1) {
                (Choice::Rock, Choice::Scissors)
                | (Choice::Paper, Choice::Rock)
                | (Choice::Scissors, Choice::Paper) => Outcome::Lose,
                (Choice::Rock, Choice::Rock)
                | (Choice::Paper, Choice::Paper)
                | (Choice::Scissors, Choice::Scissors) => Outcome::Draw,
                (Choice::Rock, Choice::Paper)
                | (Choice::Paper, Choice::Scissors)
                | (Choice::Scissors, Choice::Rock) => Outcome::Win,
            }
            .score()
    }
}

impl Score for ChoiceOutcome {
    fn score(&self) -> u32 {
        self.1.score()
            + match (&self.0, &self.1) {
                (Choice::Rock, Outcome::Draw)
                | (Choice::Paper, Outcome::Lose)
                | (Choice::Scissors, Outcome::Win) => Choice::Rock,
                (Choice::Paper, Outcome::Draw)
                | (Choice::Scissors, Outcome::Lose)
                | (Choice::Rock, Outcome::Win) => Choice::Paper,
                (Choice::Rock, Outcome::Lose)
                | (Choice::Paper, Outcome::Win)
                | (Choice::Scissors, Outcome::Draw) => Choice::Scissors,
            }
            .score()
    }
}

impl Score for Choice {
    fn score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl From<&str> for Choice {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("unknown input"),
        }
    }
}

impl From<&str> for Outcome {
    fn from(input: &str) -> Self {
        match input {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("unknown input"),
        }
    }
}

pub fn run() -> std::io::Result<Timing> {
    let buffer = include_str!("../input/02/input.txt");

    // split by double newline first, to create an iterator over each group of calories
    let start = Instant::now();
    let choice_choice_score: u32 = buffer
        .split("\n")
        .map(|raw| {
            let splitted = raw.split_once(' ').unwrap();
            ChoiceChoice::new(splitted).score()
        })
        .sum();
    let elapsed_part1 = start.elapsed();
    println!("Total score pair of choices: {}", choice_choice_score);

    let start = Instant::now();
    let choice_outcome_score: u32 = buffer
        .split("\n")
        .map(|raw| {
            let splitted = raw.split_once(' ').unwrap();
            ChoiceOutcome::new(splitted).score()
        })
        .sum();
    let elapsed_part2 = start.elapsed();
    println!(
        "Total score pair of choice | outcome: {}",
        choice_outcome_score
    );

    Ok(Timing(elapsed_part1, elapsed_part2))
}
