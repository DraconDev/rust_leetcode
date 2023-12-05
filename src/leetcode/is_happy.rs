pub fn is_happy(n: i32) -> bool {
    let mut digits = n as u32;
    let mut count = 0;
    loop {
        match digits {
            1 => return true,
            _ => {
                let mut sum = 0;
                let digits_as_str = digits.to_string();
                for c in digits_as_str.chars() {
                    sum += c.to_digit(10).unwrap().pow(2)
                }
                digits = sum;
                count += 1;
                if count > 20 {
                    return false;
                }
            }
        }
    }
}
