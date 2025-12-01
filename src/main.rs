mod problems;

fn main() {
    let input = include_str!("../input/day1.txt");

    let start = std::time::Instant::now();

    let result = problems::day1::part2(input);

    let duration = start.elapsed();

    println!("answer: {}\nduration: {:?}", result, duration);
}
