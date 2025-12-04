use std::{collections::HashSet, ops::RangeInclusive};

use fxhash::{FxBuildHasher, FxHashSet, FxHasher64};

#[allow(unused)]
pub fn part1(input: &str) -> u64 {
    input
        .trim_end()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            sum_invalid_ids_p1(start, end)
        })
        .sum()
}

// credit to Andrew Tweddle
// https://github.com/AndrewTweddle/CodingExercises/blob/master/AdventOfCode/aoc2025_rs/src/bin/day2_part1_shorter.rs
fn sum_invalid_ids_p1(start: u64, end: u64) -> u64 {
    let mut pow = 1;
    while pow * pow <= start {
        pow *= 10;
    }

    let mut half_start = start / pow;

    if half_start < pow / 10 {
        half_start = pow / 10;
    } else if half_start * (pow + 1) < start {
        half_start += 1;
    }

    let mut half_end = pow - 1;
    let mut sum = 0;

    while half_start * (pow + 1) <= end {
        if half_end * (pow + 1) > end {
            half_end = end / pow;
            if half_end * (pow + 1) > end {
                half_end -= 1;
            }
        }

        // add halfs * masks
        sum += (half_start..=half_end).map(|i| i * (pow + 1)).sum::<u64>();

        half_start = pow;
        pow *= 10;
        half_end = pow - 1;
    }

    sum
}

// credit to Andrew Tweddle
// https://github.com/AndrewTweddle/CodingExercises/blob/master/AdventOfCode/aoc2025_rs/src/bin/day2_part1_shorter.rs

pub fn part2(input: &str) -> u64 {
    let mut invalid_ids = HashSet::with_capacity_and_hasher(1000, FxBuildHasher::default());

    input.trim_end().split(',').for_each(|range| {
        let (start, end) = range.split_once('-').unwrap();
        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();
        insert_invalid_ids(&mut invalid_ids, start, end)
    });

    invalid_ids.iter().sum::<u64>()
}

fn insert_invalid_ids(repeating_numbers: &mut HashSet<u64, FxBuildHasher>, start: u64, end: u64) {
    let max_digit_count = end.to_string().len() / 2;
    for digit_count in 1..=max_digit_count {
        insert_invalid_ids_with_digit_count(repeating_numbers, start, end, digit_count);
    }
}

fn insert_invalid_ids_with_digit_count(
    repeating_numbers: &mut HashSet<u64, FxBuildHasher>,
    start: u64,
    end: u64,
    digit_count: usize,
) {
    if start > end {
        return;
    }

    // The repeater is the number to multiply a sequence of
    // `digit_count` digits by to repeat it enough times
    // so that it covers all the digits of `start`.
    let mut repeater = 1;
    let first_pow_10 = 10_u64.pow(digit_count as u32);

    // Find the power of 10 that extracts the last set
    // of up to `digit_count` digits from `start`
    let mut last_pow_10 = 1;
    while last_pow_10 * first_pow_10 <= start {
        last_pow_10 *= first_pow_10;
        repeater += last_pow_10;
    }

    // Check if there were enough groups of digits for any repetitions to occur.
    if last_pow_10 == 1 {
        // There weren't enough digits in `start` to evenly divide by the number of digits.
        // So start at the first number that has the required number of digits.
        // We do this because `end` might have enough digits, even though `start` doesn't.
        insert_invalid_ids_with_digit_count(repeating_numbers, first_pow_10, end, digit_count);
        return;
    }

    let mut init = start / last_pow_10;
    if init < first_pow_10 / 10 {
        // There aren't enough digits in `start` to evenly divide by the number of digits.
        // So start at the first number that has the required number of digits.
        init = first_pow_10 / 10;
    } else if init * repeater < start {
        // The start number is higher than the repetition of the first few digits,
        // So increment the first set of digits to be repeated, so that it will be in range.
        init += 1;
    }

    // Detect when we have more digits than end and return None to stop further iterations
    if init * repeater > end {
        return;
    }

    while init * repeater <= end {
        let mut fin = first_pow_10 - 1;
        if fin * repeater > end {
            fin = end / last_pow_10;
            if fin * repeater > end {
                // The end number is lower than the repetition of the last few digits.
                // So decrement the last set of digits to be repeated, so that it will be in range.
                fin -= 1;
            }
        }

        repeating_numbers.extend((init..=fin).map(|i| i * repeater));

        // Prepare for the next iteration
        init = first_pow_10 / 10;
        last_pow_10 *= first_pow_10;
        repeater += last_pow_10;
    }
}
