use day::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input.txt");
    let result = process(file).context("Process Part 2");
    println!("Day XX; Part 2: The result is {}.", result?);
    Ok(())
}
