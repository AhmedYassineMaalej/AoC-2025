#![warn(clippy::pedantic)]

mod problems;

fn main() {
    let input = include_str!("../input/day5.txt");

    let start = std::time::Instant::now();

    let result = problems::day5::part2(input);

    let duration = start.elapsed();

    println!("answer: {result}\nduration: {duration:?}");
}
