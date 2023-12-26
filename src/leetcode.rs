// use crate::*;
pub mod add_binary;
pub mod asteroid_collision;
pub mod count_characters;
pub mod decode_string;
pub mod equal_row_and_column_pairs;
pub mod length_of_last_word;
pub mod min_time_to_visit_all_points;
pub mod my_sqrt;
pub mod remove_element;
// mod *;
pub mod binary_tree;
pub mod delete_middle;

pub mod buy_choc;
pub mod can_visit_all_rooms;
pub mod destination_city;
pub mod find_circle_num;
pub mod find_special_integer;
pub mod food_ratings;
pub mod image_smoother;
pub mod is_happy;
pub mod largest_good_integer;
pub mod largest_odd_number;
pub mod linked_list;
pub mod max_difference;
pub mod max_product;
pub mod max_width_of_vertical_area;
pub mod number_of_matches;
pub mod ones_minus_zeros;
pub mod predict_party_victory;
pub mod recent_counter;
pub mod reverse_linked_list;
pub mod solution;
pub mod special_position;
pub mod transpose_matrix;
pub mod valid_anagram;

pub use crate::*;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut score = 0;
        for c in s.chars() {
            if c == '1' {
                score += 1;
            }
        }

        let mut max = 0;
        for c in s.chars().take(s.len() - 1) {
            if c == '0' {
                score += 1;
            } else {
                score -= 1;
            }
            if score > max {
                max = score
            }
        }
        max
    }
}

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut visit = std::collections::HashSet::new();
        let mut current = (0, 0);
        for c in path.chars() {
            visit.insert(current);
            match c {
                'N' => {
                    current.1 += 1;
                }
                'S' => {
                    current.1 -= 1;
                }
                'E' => {
                    current.0 += 1;
                }
                'W' => {
                    current.0 -= 1;
                }
                _ => {}
            }
            if visit.contains(&current) {
                return true;
            }
        }
        false
    }
}

// impl Solution {
//     pub fn get_row(row_index: i32) -> Vec<i32> {
//         let mut row = vec![1];
//         for i in 0..row_index {
//             let mut temp: Vec<i32> = Vec::new();
//             for j in 0..row.len() + 1 {
//                 temp.push(row.get(j - 1).unwrap_or(&0) + row.get(j).unwrap_or(&0));
//             }
//             row = temp;
//         }
//         row
//     }
// }

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        // 1 0 0 0
        // 1 1 0 0
        // 1 2 1 0
        // 1 3 3 1
        // f(x, y) = f(x - 1, y - 1) + f(x, y - 1)

        let row_index = row_index as usize;
        let mut arr = vec![0; row_index + 1];
        arr[0] = 1;

        for i in 1..arr.len() {
            for j in (1..=i).rev() {
                arr[j] = arr[j] + arr[j - 1]
            }
        }

        arr
    }
}

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut count = (0, 0);
        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                if c == '1' {
                    count.0 += 1;
                } else if c == '0' {
                    count.1 += 1;
                }
            } else {
                if c == '0' {
                    count.0 += 1;
                } else if c == '1' {
                    count.1 += 1;
                }
            }
        }
        if count.0 > count.1 {
            count.1
        } else {
            count.0
        }
    }
}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut elements = std::collections::HashMap::new();
        for e in &nums {
            let count = elements.entry(e).or_insert(0);
            *count += 1;
            if *count > nums.len() / 2 {
                return *e;
            }
        }
        0
    }
}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut count = 0;
        let chars: Vec<char> = s.chars().collect();
        for (i, c) in s.chars().enumerate() {
            match c {
                '0' => {
                    if i > 0 && (chars[i - 1] == '1' || chars[i - 1] == '2') {
                        count += 1
                    } else {
                        return 0;
                    }
                }
                _ => count += 1,
            }
        }

        count
    }
}
