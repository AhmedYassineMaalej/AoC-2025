use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasherDefault,
};

use fxhash::{FxBuildHasher, FxHasher};

fn propagate_beam(
    mut row: usize,
    col: usize,
    max_row: usize,
    splits: &HashSet<(usize, usize), BuildHasherDefault<FxHasher>>,
    visited: &mut HashSet<(usize, usize), BuildHasherDefault<FxHasher>>,
) -> usize {
    loop {
        // skip visited cells
        if visited.contains(&(row, col)) {
            return 0;
        }

        // mark cell as visited
        visited.insert((row, col));

        if row == max_row {
            return 0;
        }

        if splits.contains(&(row, col)) {
            // split
            return 1
                + propagate_beam(row, col + 1, max_row, splits, visited)
                + propagate_beam(row, col - 1, max_row, splits, visited);
        }

        row += 1;
    }
}

#[allow(unused)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let max_row = grid.len() - 1;

    let start = grid[0].iter().position(|&c| c == b'S').unwrap();

    let mut splits = HashSet::with_capacity_and_hasher(2000, FxBuildHasher::new());

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if cell == &b'^' {
                splits.insert((row_idx, col_idx));
            }
        }
    }

    let mut visited = HashSet::with_capacity_and_hasher(10000, FxBuildHasher::new());

    propagate_beam(0, start, max_row, &splits, &mut visited)
}

#[allow(unused)]
pub fn part2(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let max_row = grid.len() - 1;

    let start = grid[0].iter().position(|&c| c == b'S').unwrap();

    let mut splits = HashSet::with_capacity_and_hasher(2000, FxBuildHasher::new());

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if cell == &b'^' {
                splits.insert((row_idx, col_idx));
            }
        }
    }

    let mut visited = HashMap::with_capacity_and_hasher(10000, FxBuildHasher::new());

    count_timelines(0, start, max_row, &splits, &mut visited)
}

fn count_timelines(
    row: usize,
    col: usize,
    max_row: usize,
    splits: &HashSet<(usize, usize), BuildHasherDefault<FxHasher>>,
    visited: &mut HashMap<(usize, usize), usize, BuildHasherDefault<FxHasher>>,
) -> usize {
    // skip visited cells
    if visited.contains_key(&(row, col)) {
        return visited[&(row, col)];
    }

    if row == max_row {
        // mark cell as visited
        visited.insert((row, col), 1);
        return 1;
    }

    // propagate down if not at splitter
    if !splits.contains(&(row, col)) {
        // mark cell as visited
        let timelines = count_timelines(row + 1, col, max_row, splits, visited);
        visited.insert((row, col), timelines);
        return timelines;
    }

    // split
    let timelines = count_timelines(row, col + 1, max_row, splits, visited)
        + count_timelines(row, col - 1, max_row, splits, visited);
    visited.insert((row, col), timelines);
    timelines
}
