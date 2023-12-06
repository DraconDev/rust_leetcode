// pub struct Solution {}

// impl Solution {
//     pub fn new() -> Self {
//         Self {}
//     }
//     pub fn total_money(n: i32) -> i32 {
//         3
//     }
// }

// pub fn total_money_count(n: i32) -> i32 {
//     if n == 0 {
//         return 0;
//     }
//     n + total_money_count(n - 1)
// }

pub fn total_money_count(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        let daily = if i % 7 == 0 { 7 } else { i % 7 };
        let count = (i - 1) / 7;
        sum += daily + count;
    }
    sum
}
