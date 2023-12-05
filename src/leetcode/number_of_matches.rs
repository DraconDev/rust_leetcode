pub fn number_of_matches(mut n: i32) -> i32 {
    let mut matches = 0;
    while n > 1 {
        matches += n / 2;
        n = n / 2 + n % 2;
    }
    matches
}
