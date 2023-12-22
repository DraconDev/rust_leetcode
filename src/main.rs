mod tests;

mod leetcode;
use leetcode::*;

use crate::leetcode::{
    largest_odd_number::largest_odd_number, reverse_linked_list::reverse_linked_list,
    solution::total_money_count,
};

pub struct Solution;

fn main() {
    let a = 1;

    let b = Some(a);

    let test = largest_odd_number("4206".to_string());

    println!("{:?}", test);
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

