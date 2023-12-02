pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    // * make map out of chars
    let mut char_map = std::collections::HashMap::new();

    for c in chars.chars() {
        *char_map.entry(c).or_insert(0) += 1;
    }
    let mut count: i32 = 0;

    'start: for word in words {
        let mut letters = std::collections::HashMap::new();
        for c in word.chars() {
            *letters.entry(c).or_insert(0) += 1;
        }
        for (letter, &letter_count) in letters.iter() {
            if letter_count > *char_map.get(letter).unwrap_or(&0) {
                continue 'start;
            }
        }
        count += word.len() as i32;
    }

    count
}
