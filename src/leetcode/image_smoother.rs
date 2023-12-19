use std::cmp::max;
use std::cmp::min;
pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (img.len(), img[0].len());
    let mut result = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            let mut sum = 0;
            let mut cnt = 0;
            for inner_i in max(i as i32 - 1, 0) as usize..=min(i + 1, m - 1) {
                for inner_j in max(j as i32 - 1, 0) as usize..=min(j + 1, n - 1) {
                    sum += img[inner_i][inner_j];
                    cnt += 1;
                }
            }
            result[i][j] = sum / cnt;

        }
    }

    return result;
}
