pub fn largest_good_integer(num: String) -> String {
    if num.len() < 3 {
        return String::new();
    }
    let chars: Vec<char> = num.chars().collect();
    let mut result: Vec<char> = Vec::new();

    for i in 0..num.len() - 2 {
        let window = &chars[i..i + 3];
        if window[0] == window[1] && window[1] == window[2] {
            if result.is_empty() || result[0] < window[0] {
                result = window.to_vec();
            }
        }
    }

    return result.iter().collect();
}

// pub fn largest_good_integer(num: String) -> String {
//     // Convert the string into a byte slice for more efficient comparison
//     let bytes = num.as_bytes();
//     let mut max_triplet = None;

//     // Iterate over the byte slice with a window of size 3
//     for window in bytes.windows(3) {
//         if window[0] == window[1] && window[1] == window[2] {
//             // Update max_triplet if the current triplet is greater
//             max_triplet = Some(max_triplet.map_or(window, |max| std::cmp::max(max, window)));
//         }
//     }

//     // If a max_triplet is found, convert it to a String, otherwise return an empty String
//     max_triplet
//         .map(|triplet| triplet.iter().map(|&c| c as char).collect())
//         .unwrap_or_else(String::new)
// }
