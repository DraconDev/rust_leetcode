mod leetcode;
mod tests;

use leetcode::*;

fn main() {
    // let test = remove_element::remove_element(&mut vec![3, 2, 2, 3], 3);

    let test = equal_row_and_column_pairs::equal_pairs(vec![
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
    ]);

    println!("{:?}", test);
}
