use std::time::Duration;

use termion::{color, style};

use aoc_22::*;
fn main() -> std::io::Result<()> {
    let mut total_duration = Duration::default();
    for (idx, day_fn) in days().into_iter().enumerate() {
        total_duration += run(idx + 1, day_fn)?
    }

    println!(
        "{}{}Total for all days: {total_duration:?}{}",
        style::Bold,
        color::Fg(color::Green),
        style::Reset
    );
    Ok(())
}
