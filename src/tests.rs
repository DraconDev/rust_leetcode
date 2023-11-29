use crate::leetcode::remove_element;

use super::*;

#[test]
fn test_add() {
    // assert_eq!(remove_element(1, 2), 3);
    assert_eq!(remove_element(&mut vec![3, 2, 2, 3], 3), 2);
}

// #[test]
// fn test_bad_add() {
//     // This assert would fire and test will fail.
//     // Please note, that private functions can be tested too!
//     assert_eq!(bad_add(1, 2), 3);
// }
