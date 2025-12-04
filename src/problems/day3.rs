#[allow(unused)]
pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let batteries = line.as_bytes();
            max_joltage::<2>(batteries)
        })
        .sum()
}

#[allow(unused)]
pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let batteries = line.as_bytes();
            max_joltage::<12>(batteries)
        })
        .sum()
}

fn max_joltage<const DIGITS: usize>(batteries: &[u8]) -> u64 {
    let mut acc = 0;
    let mut start = 0;

    for remaining_digits in (0..DIGITS).rev() {
        let end = batteries.len() - remaining_digits;

        let slice = &batteries[start + 1..end];
        let (mut max_idx, mut max_digit) = (0, 0);

        for (idx, &digit) in slice.iter().enumerate() {
            if digit > max_digit {
                max_idx = idx;
                max_digit = digit;
            }
        }

        acc = acc * 10 + u64::from(max_digit - b'0');
        start += max_idx + 1;
    }

    acc
}
