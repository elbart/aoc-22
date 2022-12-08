use std::time::Duration;
use termion::{color, style};
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

pub fn days() -> Vec<fn() -> std::io::Result<Timing>> {
    vec![
        day_01::run,
        day_02::run,
        day_03::run,
        day_04::run,
        day_05::run,
    ]
}

pub struct Timing(Duration, Duration);

pub fn run(day: usize, f: fn() -> std::io::Result<Timing>) -> std::io::Result<Duration> {
    println!(
        "{}-------------------- BEGIN Day {:02} --------------------{}",
        style::Bold,
        day,
        style::Reset
    );

    let timing = f()?;

    println!(
        "\n-- {}Part 1: {:.3?}{}",
        style::Bold,
        timing.0,
        style::Reset
    );
    println!("-- {}Part 2: {:.3?}{}", style::Bold, timing.1, style::Reset);
    println!(
        "-- {}{}total:  {:.3?}{}",
        style::Bold,
        color::Fg(color::Green),
        timing.0 + timing.1,
        style::Reset
    );
    println!(
        "{}--------------------  END Day {:02}  --------------------{}\n\n",
        style::Bold,
        day,
        style::Reset
    );

    Ok(timing.0 + timing.1)
}
