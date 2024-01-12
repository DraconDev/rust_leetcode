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

// test preorder_traversal

#[test]
fn test_preorder_traversal() {
    assert_eq!(
        Solution::preorder_traversal(build_tree()),
        vec![3, 9, 20, 15, 7]
    )
}

// test majority_element

#[test]
fn test_majority_element() {
    assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3)
}

// test num_decodings

#[test]
fn test_num_decodings() {
    assert_eq!(Solution::num_decodings(String::from("1123")), 5);
    // assert_eq!(Solution::num_decodings(String::from("226")), 3);
    // * 06
    // assert_eq!(Solution::num_decodings(String::from("06")), 0)
}

// test min_reorder

#[test]
fn test_min_reorder() {
    assert_eq!(
        Solution::min_reorder(
            6,
            vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]
        ),
        3
    );
}

// test min cost

#[test]
fn test_min_cost() {
    // assert_eq!(
    //     Solution::min_cost(String::from("abaac"), vec![31, 12, 3, 4, 5]),
    //     3
    // );
    // aaabbbabbbb
    assert_eq!(
        Solution::min_cost(
            String::from("aaabbbabbbb"),
            vec![3, 5, 10, 7, 5, 3, 5, 5, 4, 8, 1]
        ),
        26
    )
}

// test get_length_of_optimal_compression

#[test]
fn test_get_length_of_optimal_compression() {
    assert_eq!(
        Solution::get_length_of_optimal_compression(String::from("aabbaa"), 2),
        2
    )
}

// test title_to_number

#[test]
fn test_title_to_number() {
    assert_eq!(Solution::title_to_number(String::from("ZY")), 701)
}

// test remove_elements

#[test]
fn test_remove_elements() {
    assert_eq!(
        Solution::remove_elements(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 6, next: None })),
                })),
            })),
            6
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }))
    )
}

// test is_isomorphic

#[test]
fn test_is_isomorphic() {
    assert_eq!(
        Solution::is_isomorphic(String::from("egg"), String::from("add")),
        true
    )
}

// test contains_nearby_duplicate

#[test]
fn test_contains_nearby_duplicate() {
    assert_eq!(
        Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
        true
    )
}

// test max_length_between_equal_characters

#[test]
fn test_max_length_between_equal_characters() {
    // assert_eq!(
    //     Solution::max_length_between_equal_characters(String::from("aa")),
    //     -1
    // );
    assert_eq!(
        Solution::max_length_between_equal_characters(String::from("abca")),
        2
    )
}

// test find_content_children

#[test]
fn test_find_content_children() {
    assert_eq!(
        Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
        1
    )
}

// test find matrix [1,3,4,1,2,3,1]

#[test]
fn test_find_matrix() {
    assert_eq!(
        Solution::find_matrix(vec![1, 3, 4, 1, 2, 3, 1]),
        vec![vec![1, 3, 4], vec![1, 2, 3], vec![1]]
    )
}

// test number_of_beams

#[test]
fn test_number_of_beams() {
    // ["011001","000000","010100","001000"]
    assert_eq!(
        Solution::number_of_beams(vec![
            String::from("011001"),
            String::from("000000"),
            String::from("010100"),
            String::from("001000")
        ]),
        8
    )
}

// test min_operations2

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations2(vec![1, 1, 2, 2, 2, 2]), 3)
}

// test binary_search

#[test]
fn test_binary_search() {
    assert_eq!(Solution::binary_search(vec![-1, 0, 2, 5, 9, 12], 2), -1);

    // assert_eq!(Solution::binary_search(vec![5], -5), 0)
}

// test length_of_lis

#[test]
fn test_length_of_lis() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    // assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4)
}

// test search_matrix

#[test]
fn test_search_matrix() {
    assert_eq!(
        Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            51
        ),
        true
    )
}

// test min_eating_speed

#[test]
fn test_min_eating_speed() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4)
    // [30,11,23,4,20]
    // assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30)
}

// test find_min

#[test]
fn test_find_min() {
    assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1)
}

// test leaf_similar

#[test]
fn test_leaf_similar() {
    assert_eq!(
        Solution::leaf_similar(
            crate::leetcode::binary_tree::build_tree(),
            crate::leetcode::binary_tree::build_tree()
        ),
        true
    )
}

// test search

#[test]
fn test_search() {
    assert_eq!(Solution::search(vec![3, 1], 1), 1);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![3, 4, 0, 1, 2], 1), 3);
    // assert_eq!(Solution::search(vec![1], 1), 0);
    assert_eq!(Solution::search(vec![3, 5, 1], 3), 0);
    assert_eq!(Solution::search(vec![1], 0), -1);
    assert_eq!(Solution::search(vec![1], 2), -1);
    // assert_eq!(Solution::search(vec![1, 3], 3), 1);
}

// test max_ancestor_diff

// #[test]
// fn test_max_ancestor_diff() {
//     assert_eq!(
//         Solution::max_ancestor_diff(crate::leetcode::binary_tree::build_tree(),),
//         1
//     )
// }
