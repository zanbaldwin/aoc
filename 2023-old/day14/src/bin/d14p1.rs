use day14::{error::Error, part1::process};

#[tracing::instrument]
fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("Day 14; Part 1: The result is {result}.");
    Ok(())
}
