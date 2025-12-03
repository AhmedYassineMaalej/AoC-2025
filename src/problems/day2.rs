use std::ops::RangeInclusive;

const POW_10: [usize; 12] = {
    let mut pow10 = [0; 12];

    let mut pow = 1;
    let mut i = 0;
    while i < pow10.len() {
        pow10[i] = pow;
        pow *= 10;
        i += 1;
    }

    pow10
};

const CHUNK_SIZES: [&[usize]; 11] = [
    &[],
    &[],
    &[1],
    &[1],
    &[1, 2],
    &[1],
    &[1, 2, 3],
    &[1],
    &[1, 2, 4],
    &[1, 3],
    &[1, 5],
];

fn partition_by_len(
    start: usize,
    end: usize,
) -> impl Iterator<Item = (RangeInclusive<usize>, usize)> {
    let start_len = start.ilog10();
    let end_len = end.ilog10();

    if start_len == end_len {
        return vec![(start..=end, start_len as usize + 1)].into_iter();
    }

    let len = start_len as usize + 1;
    let midpoint = POW_10[len];

    vec![(start..=midpoint, len), (midpoint..=end, len + 1)].into_iter()
}

fn sum_invalid_ids_p1((range, len): (RangeInclusive<usize>, usize)) -> usize {
    if !len.is_multiple_of(2) {
        return 0;
    }

    let div = POW_10[len / 2];
    range.filter(|id| id / div == id % div).sum::<usize>()
}

pub fn part1(input: &str) -> usize {
    input
        .trim_end()
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            partition_by_len(start, end)
        })
        .map(sum_invalid_ids_p1)
        .sum()
}

#[allow(unused)]
pub fn part2(input: &str) -> usize {
    input
        .trim_end()
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();

            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            partition_by_len(start, end)
        })
        .map(sum_invalid_ids_p2)
        .sum::<usize>()
}

fn sum_invalid_ids_p2((range, len): (RangeInclusive<usize>, usize)) -> usize {
    let mut sum = 0;

    for id in range {
        for &chunk_size in CHUNK_SIZES[len] {
            let div = POW_10[chunk_size];

            if invalid_id_p2(id, div) {
                assert!(invalid_id(&id.to_string()));
                sum += id;
                break;
            }
        }
    }
    sum
}

fn invalid_id_p2(mut id: usize, div: usize) -> bool {
    let pattern = id % div;
    while id > 0 && id % div == pattern {
        id /= div;
    }

    id == 0
}

fn invalid_id(id: &str) -> bool {
    for i in 1..id.len() {
        if !id.len().is_multiple_of(i) {
            continue;
        }

        let substr = &id[..i];
        if id == substr.repeat(id.len() / i) {
            return true;
        }
    }

    false
}
