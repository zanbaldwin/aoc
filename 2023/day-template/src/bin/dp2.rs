use day::{error::Error, part2::process};

#[tracing::instrument]
fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let file = include_str!("../../input.txt");
    let result = process(file)?;
    println!("Day XX; Part 2: The result is {result}.");
    Ok(())
}
