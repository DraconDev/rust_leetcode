use std::collections::HashMap;

// pub fn special_position(mat: Vec<Vec<i32>>) -> i32 {
//     // make maps for x and y coords
//     let mut x_map: HashMap<usize, Vec<usize>> = HashMap::new();
//     let mut y_map: HashMap<usize, Vec<usize>> = HashMap::new();

//     for i in 0..mat.len() {
//         for j in 0..mat[0].len() {
//             if mat[i][j] == 1 {
//                 x_map.entry(i).or_insert(Vec::new()).push(j);
//                 y_map.entry(j).or_insert(Vec::new()).push(i);
//             }
//         }
//     }

//     let mut count = 0;

//     for (k, v) in x_map.iter() {
//         if v.len() == 1 && y_map.get(&v[0]).unwrap().len() == 1 {
//             count += 1;
//         }
//     }
//     count
// }

pub fn special_position(mat: Vec<Vec<i32>>) -> i32 {
    let rows = mat.len();
    let cols = mat[0].len();
    let mut row_counts = vec![0; rows];
    let mut col_counts = vec![0; cols];
    let mut count = 0;

    // First pass to count '1's in each row and column.
    for i in 0..rows {
        for j in 0..cols {
            if mat[i][j] == 1 {
                row_counts[i] += 1;
                col_counts[j] += 1;
            }
        }
    }

    // Second pass to find special elements.
    for i in 0..rows {
        for j in 0..cols {
            if mat[i][j] == 1 && row_counts[i] == 1 && col_counts[j] == 1 {
                count += 1;
            }
        }
    }

    count
}
