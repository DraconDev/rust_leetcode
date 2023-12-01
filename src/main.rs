mod leetcode;
mod tests;

use leetcode::*;

fn main() {
    let test = add_binary::add_binary("11".to_string(), "1".to_string());

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
