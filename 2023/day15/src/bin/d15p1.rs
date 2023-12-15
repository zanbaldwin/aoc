use day15::{error::Error, part1::process};

#[tracing::instrument]
fn main() -> Result<(), Error<&'static str>> {
    tracing_subscriber::fmt::init();
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("Day 15; Part 1: The result is {result}.");
    Ok(())
}
