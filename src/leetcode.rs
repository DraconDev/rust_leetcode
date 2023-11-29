pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut right = nums.len() - 1;
    let mut left = nums.len() - 1;

    while left != 0 {
        if nums[left] == val {
            nums[left] = nums[right];
            right -= 1;
        }
        left -= 1;
    }
    (right as i32) - 1
}
