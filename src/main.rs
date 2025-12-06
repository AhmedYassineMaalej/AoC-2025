#![warn(clippy::pedantic)]

mod problems;

fn main() {
    let input = include_str!("../input/day6.txt");

    let start = std::time::Instant::now();

    let result = problems::day6::part1(input);

    let duration = start.elapsed();

    println!("answer: {result}\nduration: {duration:?}");
}
