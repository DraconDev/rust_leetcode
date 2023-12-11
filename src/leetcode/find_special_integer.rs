pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    // make hashmap
    let mut map = std::collections::HashMap::new();
    for e in &arr {
        let count = map.entry(e).or_insert(0);
        *count += 1;
    }
    // iterate map
    for (k, v) in map {
        if v > arr.len() / 4 {
            return *k;
        }
    }
    0
}
