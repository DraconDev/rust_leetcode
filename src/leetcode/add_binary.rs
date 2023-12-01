// pub fn add_binary(a: String, b: String) -> String {
//     // Convert binary string to u128
//     let num_a = u128::from_str_radix(&a, 2).expect("Invalid binary input for a");
//     let num_b = u128::from_str_radix(&b, 2).expect("Invalid binary input for b");

//     // Add the two numbers
//     let sum = num_a + num_b;

//     // Convert the sum back to a binary string
//     format!("{:b}", sum)
// }

pub fn add_binary(a: String, b: String) -> String {
    let mut result: String = String::new();
    let mut longer = a.chars().rev().collect::<String>();
    let mut shorter = b.chars().rev().collect::<String>();
    let mut remained = 0;
    if shorter.len() > longer.len() {
        std::mem::swap(&mut longer, &mut shorter);
    };
    for i in 0..=longer.len() {
        let mut sum = remained;
        if longer.len() > i {
            sum += longer.chars().nth(i).unwrap().to_digit(2).unwrap();
        }
        if shorter.len() > i {
            sum += shorter.chars().nth(i).unwrap().to_digit(2).unwrap();
        }
        if sum >= 2 {
            remained = 1;
        } else {
            remained = 0;
        }
        result.push(char::from_digit(sum % 2, 2).unwrap());
    }
    return result.chars().rev().collect();
}
