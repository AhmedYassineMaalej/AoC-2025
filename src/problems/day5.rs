use std::collections::HashSet;

pub fn part1(input: &str) -> u64 {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();
            start..=end
        })
        .collect::<Vec<_>>();

    let ids = ids
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u64>>();

    let mut count = 0;
    for id in ids {
        for range in &ranges {
            if range.contains(&id) {
                count += 1;
                break;
            }
        }
    }

    count
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        !(self.start > other.end || self.end < other.start)
    }
}

pub fn part2(input: &str) -> u64 {
    let (ranges, _ids) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();

            Range { start, end }
        })
        .collect::<Vec<_>>();

    while let Some((i, j)) = find_overlap(&ranges) {
        let r1 = ranges.remove(j);
        let r2 = ranges.remove(i);

        let r3 = Range {
            start: r1.start.min(r2.start),
            end: r1.end.max(r2.end),
        };
        ranges.push(r3);
    }

    let mut count = 0;

    for range in ranges {
        count += range.end - range.start + 1;
    }

    count
}

fn find_overlap(ranges: &[Range]) -> Option<(usize, usize)> {
    for (i, r1) in ranges.iter().enumerate() {
        for (j, r2) in ranges.iter().enumerate().skip(i + 1) {
            if r1.overlaps(r2) {
                return Some((i, j));
            }
        }
    }

    None
}
