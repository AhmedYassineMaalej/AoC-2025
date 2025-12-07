fn propagate_beam(
    mut row: usize,
    col: usize,
    row_count: usize,
    splits: &Vec<Vec<bool>>,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    loop {
        // skip visited cells
        if visited[row][col] {
            return 0;
        }

        // mark cell as visited
        visited[row][col] = true;

        if row == row_count - 1 {
            return 0;
        }

        if splits[row][col] {
            // split
            return 1
                + propagate_beam(row, col + 1, row_count, splits, visited)
                + propagate_beam(row, col - 1, row_count, splits, visited);
        }

        row += 1;
    }
}

#[allow(unused)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let start = grid[0].iter().position(|&c| c == b'S').unwrap();

    let mut splits = vec![vec![false; cols]; rows];

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if cell == &b'^' {
                splits[row_idx][col_idx] = true;
            }
        }
    }

    let mut visited = vec![vec![false; cols]; rows];

    propagate_beam(0, start, rows, &splits, &mut visited)
}

#[allow(unused)]
pub fn part2(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let start = grid[0].iter().position(|&c| c == b'S').unwrap();

    let mut splits = vec![vec![false; cols]; rows];

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if cell == &b'^' {
                splits[row_idx][col_idx] = true;
            }
        }
    }

    let mut visited = vec![vec![0usize; cols]; rows];

    count_timelines(0, start, rows, &splits, &mut visited)
}

fn count_timelines(
    row: usize,
    col: usize,
    row_count: usize,
    splits: &Vec<Vec<bool>>,
    visited: &mut Vec<Vec<usize>>,
) -> usize {
    // skip visited cells
    if visited[row][col] != 0 {
        return visited[row][col];
    }

    if row == row_count - 1 {
        // mark cell as visited
        visited[row][col] = 1;
        return 1;
    }

    // propagate down if not at splitter
    if !splits[row][col] {
        // mark cell as visited
        let timelines = count_timelines(row + 1, col, row_count, splits, visited);
        visited[row][col] = timelines;
        return timelines;
    }

    // split
    let timelines = count_timelines(row, col + 1, row_count, splits, visited)
        + count_timelines(row, col - 1, row_count, splits, visited);
    visited[row][col] = timelines;
    timelines
}
