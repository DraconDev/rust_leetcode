pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut a = 0;
    let mut b = 0;
    for i in 0..nums.len() {
        if nums[i] - a > 0 && nums[i] - a >= nums[i] - b {
            a = nums[i];
        } else if nums[i] - b > 0 && nums[i] - b >= nums[i] - a{
            b = nums[i];
        }
    }
    (a - 1) * (b - 1)
}
