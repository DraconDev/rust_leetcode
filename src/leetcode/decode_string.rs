// pub fn decode_string(s: String) -> String {
//     let mut stack = String::new();

//     let mut index = 0;
//     while index < s.len() {
//         let c = s.chars().nth(index).unwrap();
//         if c.is_digit(10) {
//             let mut slice = &s[index + 2..];

//             let mut count = c.to_digit(10).unwrap();

//             for (i, c) in slice.char_indices() {
//                 if c == ']' {
//                     slice = &slice[..i];
//                     break;
//                 }
//                 index += 1;
//             }
//             index += 3;
//             for _ in 0..count {
//                 stack += slice;
//             }
//         } else {
//             stack.push(c);
//             index += 1;
//         }
//     }
//     stack
// }

pub fn decode_string(s: String) -> String {
    let mut stack: Vec<(usize, String)> = Vec::new();
    let (mut n, mut str) = (0, String::new());
    for c in s.chars() {
        match c {
            '[' => {
                stack.push((n, str.clone()));
                n = 0;
                str.clear();
            }
            ']' => {
                if let Some(last) = stack.pop() {
                    str = last.1 + str.repeat(last.0).as_str();
                }
            }
            '0'..='9' => n = n * 10 + (c as u8 - b'0') as usize,
            c => str.push(c),
        }
    }
    str
}
