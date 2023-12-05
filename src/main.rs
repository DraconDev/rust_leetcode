mod tests;

mod leetcode;
use leetcode::*;

use crate::leetcode::recent_counter::RecentCounter;

fn main() {
    // let mut recent_counter = RecentCounter::new();
    // let mut test = recent_counter.ping(15);
    // test = recent_counter.ping(11115);

    // let test = largest_good_integer::largest_good_integer("6111777133339".to_string());

    // let test = predict_party_victory::predict_party_victory("DDRRRR".to_string());

    // let test = is_happy::is_happy(19);
    let test = number_of_matches::number_of_matches(14);
    // let test = asteroid_collision::asteroid_collision(vec![-2, 2, 1, -2]);
    // let test = min_time_to_visit_all_points::min_time_to_visit_all_points(vec![
    //     vec![1, 1],
    //     vec![3, 4],
    //     vec![3, 2],
    // ] as Vec<Vec<i32>>);
    // let test = count_characters::count_characters(
    //     vec![
    //         "cat".to_string(),
    //         "bt".to_string(),
    //         "hat".to_string(),
    //         "tree".to_string(),
    //     ],
    //     "atach".to_string(),
    // );
    // let test = my_sqrt::my_sqrt(452345);

    // let test = decode_string::decode_string("3[a]2[bc]".to_string());

    // let v1 = vec![1, 2, 3];

    // for val in &v1 {
    //     println!("Got: {}", val);
    // }

    // let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    // let test = add_binary::add_binary("11".to_string(), "1".to_string());

    // let test = remove_element::remove_element(&mut vec![3, 2, 2, 3], 3);
    // length_of_last_word::length_of_last_word("Hello World".to_string());

    // let test = equal_row_and_column_pairs::equal_pairs(vec![
    //     vec![1, 2, 3, 4],
    //     vec![1, 2, 3, 4],
    //     vec![1, 2, 3, 4],
    //     vec![1, 2, 3, 4],
    // ]);

    println!("{:?}", test);
}
