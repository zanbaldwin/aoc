use day14::{error::Error, part2::process};

#[tracing::instrument]
fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let file = include_str!("../../input.txt");
    let result = process(file, 1_000_000_000)?;
    println!("Day 14; Part 2: The result is {result}.");
    Ok(())
}
