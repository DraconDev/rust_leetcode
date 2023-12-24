use crate::{leetcode::binary_tree::build_tree, *};

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
fn test_special_position() {
    // assert_eq!(
    //     crate::leetcode::special_position::num_special(vec![
    //         vec![1, 0, 0],
    //         vec![0, 1, 0],
    //         vec![0, 0, 1]
    //     ]),
    //     3
    // );
    assert_eq!(
        crate::leetcode::special_position::special_position(vec![
            vec![1, 0, 0],
            vec![0, 0, 1],
            vec![1, 0, 0]
        ]),
        1
    );
}

// test for longest zig zag
// #[test]
// fn test_longest_zig_zag() {
//     assert_eq!(
//         crate::leetcode::binary_tree::longest_zig_zag(build_tree()),
//         3
//     )
// }

// test for ones minus zeros
//  [[0,1,1],[1,0,1],[0,0,1]]
#[test]
fn test_ones_minus_zeros() {
    assert_eq!(
        crate::leetcode::ones_minus_zeros::ones_minus_zeros(vec![
            vec![0, 1, 1],
            vec![1, 0, 1],
            vec![0, 0, 1]
        ]),
        // [[0,0,4],[0,0,4],[-2,-2,2]]
        vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]]
    );
}

//  test lowest_common_ancestor
// #[test]
// fn test_lowest_common_ancestor() {
//     let tree = crate::leetcode::binary_tree::build_tree();
//     assert_eq!(
//         crate::leetcode::binary_tree::lowest_common_ancestor(tree, None, None),
//         None
//     )
// }

// test for dest_city
//  [["London","New York"],["New York","Lima"],["Lima","Sao Paulo"]]

#[test]
fn test_dest_city() {
    assert_eq!(
        crate::leetcode::destination_city::dest_city(vec![
            vec![String::from("London"), String::from("New York")],
            vec![String::from("New York"), String::from("Lima")],
            vec![String::from("Lima"), String::from("Sao Paulo")]
        ]),
        "Sao Paulo".to_string()
    )
}

#[test]
fn text_max_product_difference() {
    assert_eq!(
        crate::leetcode::max_difference::max_product_difference(vec![5, 6, 2, 7, 4]),
        34
    )
}

// test is balanced
// #[test]
// fn test_is_balanced() {
//     assert_eq!(
//         crate::leetcode::binary_tree::is_balanced(build_tree()),
//         true
//     )
// }

// test image_smoother
#[test]
fn test_image_smoother() {
    assert_eq!(
        crate::leetcode::image_smoother::image_smoother(vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1]
        ]),
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
    )
}
// test buy_choco

#[test]
fn test_buy_choco() {
    assert_eq!(crate::leetcode::buy_choc::buy_choco(vec![1, 1, 2, 3], 1), 2)
}

// test can_visit_all_rooms
#[test]
fn test_can_visit_all_rooms() {
    // assert_eq!(
    //     crate::leetcode::can_visit_all_rooms::can_visit_all_rooms(vec![
    //         vec![1, 3],
    //         vec![3, 0, 1],
    //         vec![2],
    //         vec![0]
    //     ]),
    //     false
    // );

    // testcase [[4],[3],[],[2,5,7],[1],[],[8,9],[],[],[6]]
    assert_eq!(
        crate::leetcode::can_visit_all_rooms::can_visit_all_rooms(vec![
            vec![4],
            vec![3],
            vec![],
            vec![2, 5, 7],
            vec![1],
            vec![],
            vec![8, 9],
            vec![],
            vec![],
            vec![6]
        ]),
        false
    )
}

// test max_width_of_vertical_area Input: points = [[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]] Output: 3

use crate::max_width_of_vertical_area::max_width_of_vertical_area;

#[test]
fn test_max_width_of_vertical_area() {
    assert_eq!(
        max_width_of_vertical_area(vec![
            vec![3, 1],
            vec![9, 0],
            vec![1, 0],
            vec![1, 4],
            vec![5, 3],
            vec![8, 8]
        ]),
        3
    )
}

// test find_circle_num

#[test]
fn test_find_circle_num() {
    assert_eq!(
        Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
        2
    )
}

// test min_depth

// #[test]
// fn test_min_depth() {
//     // make example tree
//     let tree: Option<std::rc::Rc<std::cell::RefCell<binary_tree::TreeNode>>> =
//         crate::leetcode::binary_tree::build_tree();
//     assert_eq!(Solution::min_depth(tree), 2)
// }

// test is_path_crossing

#[test]
fn test_is_path_crossing() {
    assert_eq!(Solution::is_path_crossing(String::from("NESWW")), true)
}

//  test has_path_sum

#[test]
fn test_has_path_sum() {
    assert_eq!(Solution::has_path_sum(build_tree(), 5), true)
}

// test get_row

#[test]
fn test_get_row() {
    assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1])
}
