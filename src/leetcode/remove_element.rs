// pub fn remove_element(numbers: &mut Vec<i32>, val: i32) -> i32 {
//     let mut left = numbers.len();
//     let mut right = numbers.len() - 1;

//     while left > 0 {
//         left -= 1;
//         if numbers[left] != val {
//             continue;
//         }
//         numbers[left] = numbers[right];
//         right -= 1;
//         numbers.pop();
//     }
//     return numbers.len() as i32;
// }

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    return nums.len() as i32;
}
