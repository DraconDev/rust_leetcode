// pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
//     let mut char_map = std::collections::HashMap::new();

//     for c in chars.chars() {
//         *char_map.entry(c).or_insert(0) += 1;
//     }
//     let mut count: i32 = 0;

//     'start: for word in words {
//         let mut letters = std::collections::HashMap::new();
//         for c in word.chars() {
//             *letters.entry(c).or_insert(0) += 1;
//         }
//         for (letter, &letter_count) in letters.iter() {
//             if letter_count > *char_map.get(letter).unwrap_or(&0) {
//                 continue 'start;
//             }
//         }
//         count += word.len() as i32;
//     }
//     count
// }

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut char_map = std::collections::HashMap::new();

    for c in chars.chars() {
        *char_map.entry(c).or_insert(0) += 1;
    }
    let mut count: i32 = 0;

    'words: for word in words {
        let mut temp_map = char_map.clone();
        for c in word.chars() {
            let entry = temp_map.entry(c).or_insert(0);
            if *entry == 0 {
                continue 'words;
            }
            *entry -= 1;
        }
        count += word.len() as i32;
    }
    count
}
