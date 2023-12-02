use day01::part2::process;
use miette::Context;
use load_file::load_str;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let file = load_str!("../../input.txt");
    let result = process(file).context("Process Part 2");
    println!("Day 01; Part 2: The result is {}.", result?);
    Ok(())
}