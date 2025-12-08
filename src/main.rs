#![warn(clippy::pedantic)]

mod problems;

fn main() {
    let input = include_str!("../input/day8.txt");

    let start = std::time::Instant::now();

    let result = problems::day8::part2(input);

    let duration = start.elapsed();

    println!("answer: {result:?}\nduration: {duration:?}");
}
