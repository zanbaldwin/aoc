use day::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input.txt");
    let result = process(file).context("Process Part 1");
    println!("Day XX; Part 1: The result is {}.", result?);
    Ok(())
}
