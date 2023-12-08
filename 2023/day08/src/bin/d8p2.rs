use day08::{part2::process, Error};

#[tracing::instrument]
fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("Day 08; Part 2: The result is {result}.");
    Ok(())
}
