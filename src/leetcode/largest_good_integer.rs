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
