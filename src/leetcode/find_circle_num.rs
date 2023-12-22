// use std::collections::HashSet;

// pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
//     let mut visited = HashSet::new();
//     let mut count = 0;

//     for i in 0..is_connected.len() {
//         if !visited.contains(&i) {
//             dfs(&is_connected, i, &mut visited);
//             count += 1; // Found a new province.
//         }
//     }

//     count
// }

// pub fn dfs(is_connected: &Vec<Vec<i32>>, i: usize, visited: &mut HashSet<usize>) {
//     for j in 0..is_connected.len() {
//         if is_connected[i][j] == 1 && !visited.contains(&j) {
//             visited.insert(j);
//             dfs(is_connected, j, visited); // Continue DFS on the next connected city.
//         }
//     }
// }

use std::collections::HashSet;

use super::Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visited = HashSet::new();
        let mut count = 0;

        for i in 0..is_connected.len() {
            if !visited.contains(&i) {
                Self::dfs(&is_connected, i, &mut visited); // Use Solution::dfs to refer to the associated function
                count += 1; // Found a new province.
            }
        }

        count
    }

    pub fn dfs(is_connected: &Vec<Vec<i32>>, i: usize, visited: &mut HashSet<usize>) {
        for j in 0..is_connected.len() {
            if is_connected[i][j] == 1 && !visited.contains(&j) {
                visited.insert(j);
                Self::dfs(is_connected, j, visited); // Use Solution::dfs for recursive call
            }
        }
    }
}
