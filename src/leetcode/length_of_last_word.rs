pub fn length_of_last_word(s: String) -> i32 {
    let s = s.trim();
    for (i, c) in s.chars().rev().enumerate() {
        if c == ' ' {
            return i as i32;
        }
    }
    s.len() as i32
}

// 2
pub fn length_of_last_word(s: String) -> i32 {
    let s = s.trim();
    for (i, c) in s.chars().rev().enumerate() {
        if c == ' ' {
            return i as i32;
        }
    }
    s.len() as i32
}
