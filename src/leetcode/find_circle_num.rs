use std::collections::hash_set;

pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
    let mut provinces = hash_set::HashSet::new();

    for i in 0..is_connected.len() {
        for j in 0..is_connected[i].len() {
            if is_connected[i][j] == 1 && i != j {
                provinces.insert(j);
            }
        }
    }

    if provinces.is_empty() {
        return is_connected.len() as i32;
    }

    (is_connected.len() - provinces.len() + 1) as i32
}
