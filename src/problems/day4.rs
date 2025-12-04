use std::collections::HashSet;

fn count_neighbouring_rolls(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for r in row.saturating_sub(1)..(row + 2).min(rows) {
        for c in col.saturating_sub(1)..(col + 2).min(cols) {
            if grid[r][c] {
                count += 1;
            }
        }
    }

    count
}

pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut neighbour_grid = vec![vec![0u8; cols]; rows];

    for row in 0..rows {
        for col in 0..cols {
            if !grid[row][col] {
                continue;
            }

            increment_neighbours(&mut neighbour_grid, row, col);
        }
    }

    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] && neighbour_grid[row][col] < 4 {
                count += 1;
            }
        }
    }

    count
}

fn iter_neighbours(
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (row.saturating_sub(1)..(row + 2).min(rows)).flat_map(move |row| {
        (col.saturating_sub(1)..(col + 2).min(cols)).map(move |col| (row, col))
    })
}

fn increment_neighbours(grid: &mut Vec<Vec<u8>>, row: usize, col: usize) {
    let rows = grid.len();
    let cols = grid[0].len();

    for (r, c) in iter_neighbours(row, col, rows, cols) {
        if r == row && c == col {
            continue;
        }

        grid[r][c] += 1;
    }
}

// make empty set for visited
// make a queue for cells to check
// while queue is not empty
//   if cell removed {
//      skip
//   }
//
//   cell = queue.pop()
//   if can remove cell {
//      decrement neighbouring cell neighbour count
//      add cell to removed cells
//      add neighbours to check
//   }

pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut neighbours_count = vec![vec![0; cols]; rows];
    let mut to_check: Vec<(usize, usize)> = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] {
                increment_neighbours(&mut neighbours_count, row, col);
                to_check.push((row, col));
            }
        }
    }

    let mut count = 0;
    while let Some((row, col)) = to_check.pop() {
        if !grid[row][col] {
            // cell does not contain paper roll
            continue;
        }

        // check if roll can be removed
        if neighbours_count[row][col] < 4 {
            count += 1;

            // remove roll
            grid[row][col] = false;

            // decrement neighbours and check if they can be removed
            for (n_row, n_col) in iter_neighbours(row, col, rows, cols) {
                neighbours_count[n_row][n_col] = neighbours_count[n_row][n_col].saturating_sub(1);
                to_check.push((n_row, n_col));
            }
        }
    }

    count
}
