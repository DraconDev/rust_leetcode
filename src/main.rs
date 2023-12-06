mod tests;

mod leetcode;
use leetcode::*;

use crate::leetcode::solution::total_money_count;

fn main() {
    let test = total_money_count(20);

    println!("{:?}", test);
}
