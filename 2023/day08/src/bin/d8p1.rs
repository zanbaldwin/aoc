use day08::{part1::process, Error};

#[tracing::instrument]
fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("Day 08; Part 1: The result is {result}.");
    Ok(())
}
