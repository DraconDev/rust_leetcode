pub fn my_sqrt(x: i32) -> i32 {
    let mut left: u128 = 0;
    let mut right = x as u128;
    while left <= right {
        let mid = left + (right - left) / 2;
        if mid * mid > x as u128 {
            right = mid - 1;
        } else if mid * mid < x as u128 {
            left = mid + 1;
        } else {
            return mid as i32;
        }
    }
    right as i32
}
