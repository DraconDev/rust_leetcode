pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut row = vec![0; grid.len()];
    let mut col = vec![0; grid[0].len()];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                1 => {
                    row[i] += 1;
                }
                0 => {
                    col[j] += 1;
                }
                _ => {}
            }
        }
    }

    let mut result: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                1 => {
                    let countRow = row[i] - grid.len() as i32 - row[i];
                    let countCol = col[j] - grid[0].len() as i32 - col[j];
                    result[i][j] = 1 + countRow + countCol;
                }
                0 => {
                    result[i][j] = -1 + row[i] - col[j] + grid.len() as i32 - col[j];
                }
                _ => {}
            }
        }
    }
    result
}
