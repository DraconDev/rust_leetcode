mod tests;

mod leetcode;
use leetcode::*;

use crate::leetcode::{
    largest_odd_number::largest_odd_number, reverse_linked_list::reverse_linked_list,
    solution::total_money_count,
};

fn main() {
    // let list = Some(Box::new(ListNode {
    //     val: 1,
    //     next: Some(Box::new(ListNode {
    //         val: 2,
    //         next: Some(Box::new(ListNode { val: 3, next: None })),
    //     })),
    // }));

    // let test = reverse_linked_list(list);


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
