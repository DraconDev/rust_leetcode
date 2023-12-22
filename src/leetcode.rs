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
