pub fn predict_party_victory(senate: String) -> String {
    let mut stack: Vec<char> = vec![];
    let mut queue: Vec<char> = senate.chars().collect();

    while !queue.is_empty() {
        let top = queue.remove(0);

        if stack.is_empty() || *stack.last().unwrap() == top {
            stack.push(top);
        } else {
            queue.push(stack.pop().unwrap());
        }
    }

    if stack.pop().unwrap() == 'R' {
        "Radiant".to_string()
    } else {
        "Dire".to_string()
    }
}
