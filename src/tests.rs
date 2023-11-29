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

// #[test]
// fn test_bad_add() {
//     // This assert would fire and test will fail.
//     // Please note, that private functions can be tested too!
//     assert_eq!(bad_add(1, 2), 3);
// }
