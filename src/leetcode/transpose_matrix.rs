pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let vert_len = matrix.len();
    let horizontal_len = matrix[0].len();

    let mut result = vec![vec![0; vert_len]; horizontal_len];

    for i in 0..vert_len {
        for j in 0..horizontal_len {
            result[j][i] = matrix[i][j];
        }
    }
    result
}
