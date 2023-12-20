pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let mut min1 = i32::MAX;
    let mut min2 = i32::MAX;

    for e in prices {
        if e < min1 {
            min2 = min1;
            min1 = e;
        } else if e < min2 {
            min2 = e;
        }
    }

    let cost = min1 + min2;
    if cost <= money {
        money - cost
    } else {
        money
    }
}
