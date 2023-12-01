pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::new();
    for asteroid in asteroids {
        if stack.len() == 0 || asteroid > 0 || (stack.last().unwrap() < &0 && asteroid < 0) {
            stack.push(asteroid);
            continue;
        }
        while stack.last().unwrap_or(&0) + asteroid <= 0 {
            if stack.len() == 0 || stack.last().unwrap() < &0 {
                stack.push(asteroid);
                break;
            }
            if stack.last().unwrap_or(&0) + asteroid == 0 {
                stack.pop();
                break;
            }
            stack.pop();
        }
    }
    stack
}
