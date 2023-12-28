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

use std::collections::{hash_map, HashMap};

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
        s.chars()
            .enumerate()
            .fold([1, 0], |a, (i, c)| {
                [
                    if c != '0' { a[0] + a[1] } else { 0 },
                    if i > 0
                        && (s.chars().nth(i - 1) == Some('1')
                            || (s.chars().nth(i - 1) == Some('2') && c < '7'))
                    {
                        a[0]
                    } else {
                        0
                    },
                ]
            })
            .iter()
            .sum()
    }
}

// impl Solution {
//     pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
//         let mut changed = 0;
//         let mut visited = std::collections::HashSet::new();
//         visited.insert(&0);

//         for e in connections.iter() {
//             if visited.contains(&e[1]) {
//                 visited.insert(&e[0]);
//             } else {
//                 visited.insert(&e[1]);
//                 changed += 1
//             }
//         }
//         changed
//     }
// }

use std::collections::HashSet;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph = HashMap::new();
        // Construct the graph
        for connection in &connections {
            graph
                .entry(connection[0])
                .or_insert(vec![])
                .push((connection[1], true)); // true indicates the edge is from connection[0] to connection[1]
            graph
                .entry(connection[1])
                .or_insert(vec![])
                .push((connection[0], false)); // false indicates the edge is from connection[1] to connection[0]
        }

        // DFS function to count the number of edges to be reversed
        fn dfs(
            city: i32,
            graph: &HashMap<i32, Vec<(i32, bool)>>,
            visited: &mut HashSet<i32>,
        ) -> i32 {
            visited.insert(city);
            let mut count = 0;

            if let Some(neighbors) = graph.get(&city) {
                for &(next_city, edge_reversed) in neighbors {
                    if !visited.contains(&next_city) {
                        // If the edge is reversed, it needs to be counted
                        count += if edge_reversed { 1 } else { 0 };
                        // Continue DFS traversal
                        count += dfs(next_city, graph, visited);
                    }
                }
            }

            count
        }

        let mut visited = HashSet::new();
        // Start DFS from the capital city (city 0)
        dfs(0, &graph, &mut visited)
    }
}

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut total_time = 0;
        let mut max_time = 0; // This will store the maximum time of the same color balloons
        let colors: Vec<char> = colors.chars().collect();

        for i in 0..colors.len() {
            // If this is the first balloon or the color has changed, update max_time
            if i == 0 || colors[i] != colors[i - 1] {
                max_time = needed_time[i];
            } else {
                // If the current balloon has the same color as the previous one
                // Add the lesser time to total_time and update max_time if necessary
                total_time += needed_time[i].min(max_time);
                max_time = needed_time[i].max(max_time);
            }
        }
        total_time
    }
}

impl Solution {
    fn dp(
        s: &[u8],
        k: usize,
        i: usize,
        last: u8,
        cnt: usize,
        j: usize,
        memo: &mut HashMap<(usize, u8, usize, usize), i32>,
    ) -> i32 {
        if i == s.len() {
            0
        } else if let Some(additional_len) = memo.get(&(i, last, cnt, j)) {
            *additional_len
        } else {
            let b = s[i];
            let mut rez = 1 + Self::dp(s, k, i + 1, b, 1, j, memo);
            if last == b {
                let comp = if cnt != 1 && cnt != 9 && cnt != 99 {
                    Self::dp(s, k, i + 1, b, cnt + 1, j, memo)
                } else {
                    1 + Self::dp(s, k, i + 1, b, cnt + 1, j, memo)
                };
                rez = rez.min(comp);
            }
            if j < k {
                rez = rez.min(Self::dp(s, k, i + 1, last, cnt, j + 1, memo));
            }
            memo.insert((i, last, cnt, j), rez);
            rez
        }
    }

    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        Self::dp(s.as_bytes(), k as usize, 0, b'A', 0, 0, &mut HashMap::new())
    }
}

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result: i32 = 0;

        for c in column_title.bytes() {
            result *= 26;
            result += (c as i32 - 65) + 1;
        }

        result
    }
}

// impl Solution {
//     pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
//         let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
//         let mut current = dummy_head.as_mut();

//         while current.is_some() {
//             while current.as_ref().unwrap().next.is_some()
//                 && current.as_ref().unwrap().next.as_ref().unwrap().val == val
//             {
//                 if current
//                     .as_ref()
//                     .unwrap()
//                     .next
//                     .as_ref()
//                     .unwrap()
//                     .next
//                     .is_none()
//                 {
//                     current.as_mut().unwrap().next = None;
//                 } else {
//                     current.as_mut().unwrap().next =
//                         current.as_mut().unwrap().next.as_mut().unwrap().next.take();
//                 }
//             }
//             current = current.unwrap().next.as_mut();
//         }

//         dummy_head.unwrap().next
//     }
// }

pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = None;
    let mut tail = &mut dummy;

    while let Some(mut node) = head.take() {
        head = node.next.take();

        if node.val != val {
            *tail = Some(node);
            tail = &mut tail.as_mut().unwrap().next;
        }
    }

    dummy
}
