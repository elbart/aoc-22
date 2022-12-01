use aoc_22::*;
fn main() -> std::io::Result<()> {
    for (idx, day_fn) in days().into_iter().enumerate() {
        run(idx + 1, day_fn)?
    }
    Ok(())
}
