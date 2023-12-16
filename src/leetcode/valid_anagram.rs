pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_map = std::collections::HashMap::new();
    for c in s.chars() {
        *s_map.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        let count = s_map.get_mut(&c);
        match count {
            Some(value) => {
                if *value <= 0 {
                    return false;
                }
                *value -= 1;
            }
            None => return false,
        }
    }
    true
}
