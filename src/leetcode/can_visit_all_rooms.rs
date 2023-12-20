use std::collections::{hash_map, hash_set, HashSet};

pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let mut visited = HashSet::new();
    let mut keys_stack = vec![0];

    while let Some(room_key) = keys_stack.pop() {
        if visited.insert(room_key) {
            for &key in &rooms[room_key as usize] {
                if !visited.contains(&key) {
                    keys_stack.push(key);
                }
            }
        }
    }

    visited.len() == rooms.len()
}
