use std::ops::Index;

const OPERAND_COUNT: usize = 4;

struct Lines<'a> {
    inner: &'a [u8],
    line_width: usize,
}

impl<'a> Lines<'a> {
    fn new(input: &'a str) -> Self {
        let inner = input.as_bytes();
        let mut line_width = 0; // index of first '\n'
        while inner[line_width] != b'\n' {
            line_width += 1;
        }

        Self {
            inner: input.as_bytes(),
            line_width,
        }
    }

    fn iter_nums_horizontal(&self, start: usize, end: usize) -> impl Iterator<Item = u64> {
        (0..OPERAND_COUNT).map(move |line_idx| {
            self[line_idx][start..end - 1]
                .iter()
                .filter_map(|&digit| (digit != b' ').then_some(digit - b'0'))
                .fold(0, |acc, digit| acc * 10 + u64::from(digit))
        })
    }

    fn iter_nums_vertical(&self, start: usize, end: usize) -> impl Iterator<Item = u64> {
        (0..end - 1 - start).map(move |digit_idx| {
            (0..OPERAND_COUNT)
                .map(|line_idx| self[line_idx][start..end - 1][digit_idx])
                .filter_map(|digit| (digit != b' ').then_some(digit - b'0'))
                .fold(0, |acc, digit| acc * 10 + u64::from(digit))
        })
    }
}

impl Index<usize> for Lines<'_> {
    type Output = [u8];

    #[inline]
    fn index(&self, line_idx: usize) -> &Self::Output {
        let start = line_idx * (self.line_width + 1);
        let end = line_idx + self.line_width + line_idx * self.line_width;
        &self.inner[start..end]
    }
}

#[allow(unused)]
pub fn part1(input: &str) -> u64 {
    let lines = Lines::new(input);

    let mut operator = lines[OPERAND_COUNT][0];

    let mut start = 0;
    let mut sum = 0;

    for end in 1..lines.line_width {
        if lines[OPERAND_COUNT][end] != b' ' {
            let operands = lines.iter_nums_horizontal(start, end);

            if operator == b'+' {
                sum += operands.sum::<u64>();
            } else {
                sum += operands.product::<u64>();
            }

            operator = lines[OPERAND_COUNT][end];
            start = end;
        }
    }

    let operands = lines.iter_nums_horizontal(start, lines.line_width + 1);
    if operator == b'+' {
        sum += operands.sum::<u64>();
    } else {
        sum += operands.product::<u64>();
    }

    sum
}

#[allow(unused)]
pub fn part2(input: &str) -> u64 {
    let lines = Lines::new(input);

    let mut operator = lines[OPERAND_COUNT][0];

    let mut start = 0;
    let mut sum = 0;

    for end in 1..lines.line_width {
        if lines[OPERAND_COUNT][end] != b' ' {
            let operands = lines.iter_nums_vertical(start, end);

            if operator == b'+' {
                sum += operands.sum::<u64>();
            } else {
                sum += operands.product::<u64>();
            }

            operator = lines[OPERAND_COUNT][end];
            start = end;
        }
    }

    let operands = lines.iter_nums_vertical(start, lines.line_width + 1);
    if operator == b'+' {
        sum += operands.sum::<u64>();
    } else {
        sum += operands.product::<u64>();
    }

    sum
}
