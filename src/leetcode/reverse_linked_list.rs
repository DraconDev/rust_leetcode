use crate::ListNode;

pub fn reverse_linked_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        let mut next = node.next.take();
        node.next = prev.take();
        prev = Some(node);
        current = next.take();
    }
    prev
}

pub fn pair_sum(mut head: Option<Box<ListNode>>) -> i32 {
    let mut elements = vec![];
    while let Some(mut node) = head {
        elements.push(node.val);
        head = node.next.take();
    }

    let length = elements.len();
    let mut highest = 0;
    for i in 0..length / 2 {
        let temp_sum = elements[i] + elements[length - 1 - i];
        highest = std::cmp::max(temp_sum, highest);
    }
    highest
}
