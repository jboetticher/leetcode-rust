impl Solution {
    pub fn ones_minus_zeros(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (mut ones_row, mut zeroes_row) = (vec![0; grid.len()], vec![0; grid.len()]);
        let (mut ones_col, mut zeroes_col) = (vec![0; grid[0].len()], vec![0; grid[0].len()]);

        // Initialize count calculations
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    ones_row[r] += 1;
                    ones_col[c] += 1;
                }
                else {
                    zeroes_row[r] += 1;
                    zeroes_col[c] += 1;
                }
            }
        }

        // Replace answer grid (don't need a new one)
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                grid[r][c] = ones_row[r] + ones_col[c] - zeroes_row[r] - zeroes_col[c];
            }
        }

        grid
    }
}
