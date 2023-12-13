use crate::*;

// use super::*;

#[test]
fn test_add() {
    // assert_eq!(remove_element(1, 2), 3);
    assert_eq!(
        crate::leetcode::remove_element::remove_element(&mut vec![3, 2, 2, 3], 3),
        2
    );
}

#[test]
fn test_equal_pairs() {
    assert_eq!(
        crate::leetcode::equal_row_and_column_pairs::equal_pairs(vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
        ]),
        3
    )
}

// test transpose_matrix
#[test]
fn test_transpose_matrix() {
    // assert_eq!(
    //     crate::leetcode::transpose_matrix::transpose(vec![
    //         vec![1, 2, 3],
    //         vec![4, 5, 6],
    //         vec![7, 8, 9]
    //     ]),
    //     vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    // );
    // * test for [[1,2,3],[4,5,6]]
    assert_eq!(
        crate::leetcode::transpose_matrix::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        vec![vec![1, 4], vec![2, 5], vec![3, 6]]
    );
}

#[test]
fn test_find_special_integer() {
    assert_eq!(
        crate::leetcode::find_special_integer::find_special_integer(vec![
            1, 2, 2, 6, 6, 6, 6, 7, 10
        ]),
        6
    )
}

// test max max_product
#[test]
fn test_max_product() {
    assert_eq!(
        crate::leetcode::max_product::max_product(vec![2, 3, -2, 4]),
        6
    )
}

// test special special_position
#[test]
fn test_num_special() {
    // assert_eq!(
    //     crate::leetcode::special_position::num_special(vec![
    //         vec![1, 0, 0],
    //         vec![0, 1, 0],
    //         vec![0, 0, 1]
    //     ]),
    //     3
    // );
    assert_eq!(
        crate::leetcode::special_position::num_special(vec![
            vec![1, 0, 0],
            vec![0, 0, 1],
            vec![1, 0, 0]
        ]),
        1
    );
}
