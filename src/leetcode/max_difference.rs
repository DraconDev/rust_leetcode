// pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
//     nums.sort();
//     (nums[nums.len() - 1]) * (nums[nums.len() - 2]) - (nums[0]) * (nums[1])
// }

pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
    let mut max1 = i32::MIN;
    let mut max2 = i32::MIN;
    let mut min1 = i32::MAX;
    let mut min2 = i32::MAX;

    for &e in &nums {
        if e > max1 {
            max2 = max1;
            max1 = e;
        } else if e > max2 {
            max2 = e;
        }

        // Update min1 and min2
        if e < min1 {
            min2 = min1;
            min1 = e;
        } else if e < min2 {
            min2 = e;
        }
    }
    max1 * max2 - min1 * min2
}
