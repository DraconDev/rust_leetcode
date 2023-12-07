pub fn largest_odd_number(num: String) -> String {
    let mut digits = num.chars().collect::<Vec<char>>();

    let mut count: i32 = (digits).len() as i32 - 1;
    while count >= 0 {
        if digits[count as usize].to_digit(10).unwrap() % 2 == 1 {
            return digits[0..count as usize + 1].iter().collect();
        }
        count -= 1;
    }
    String::new()
}
