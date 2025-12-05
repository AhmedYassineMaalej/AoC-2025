fn iter_neighbours(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    let neighbours = [
        (row.wrapping_sub(1), col.wrapping_sub(1)),
        (row.wrapping_sub(1), col),
        (row.wrapping_sub(1), col + 1),
        (row, col.wrapping_sub(1)),
        (row, col),
        (row, col + 1),
        (row + 1, col.wrapping_sub(1)),
        (row + 1, col),
        (row + 1, col + 1),
    ];

    neighbours.into_iter()
}

#[allow(unused)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();

    let mut count = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if cell == &b'.' {
                continue;
            }

            let mut neighbour_count = 0;
            for (n_row, n_col) in iter_neighbours(row_idx, col_idx) {
                if let Some(row) = grid.get(n_row)
                    && let Some(cell) = row.get(n_col)
                    && cell == &b'@'
                {
                    neighbour_count += 1;
                }
            }

            if neighbour_count < 5 {
                count += 1;
            }
        }
    }

    count
}

fn increment_neighbours(grid: &mut [Vec<u8>], row: usize, col: usize) {
    for (r, c) in iter_neighbours(row, col) {
        if let Some(row) = grid.get_mut(r)
            && let Some(cell) = row.get_mut(c)
        {
            *cell += 1;
        }
    }
}

#[allow(unused)]
pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut neighbours_count = vec![vec![0; cols]; rows];

    let mut to_check: Vec<(usize, usize)> = Vec::new();
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if cell == &b'.' {
                continue;
            }

            increment_neighbours(&mut neighbours_count, row_idx, col_idx);
            to_check.push((row_idx, col_idx));
        }
    }

    let mut count = 0;
    while let Some((row, col)) = to_check.pop() {
        if grid[row][col] == b'.' {
            // cell does not contain paper roll
            continue;
        }

        if neighbours_count[row][col] >= 5 {
            // roll on cell cannot be removed
            continue;
        }

        // remove roll
        grid[row][col] = b'.';
        count += 1;

        // decrement neighbours and check if they can be removed
        for (n_row, n_col) in iter_neighbours(row, col) {
            let Some(row) = grid.get(n_row) else {
                continue;
            };

            let Some(cell) = row.get(n_col) else {
                continue;
            };

            if cell == &b'.' {
                continue;
            }

            neighbours_count[n_row][n_col] = neighbours_count[n_row][n_col].saturating_sub(1);
            to_check.push((n_row, n_col));
        }
    }

    count
}
