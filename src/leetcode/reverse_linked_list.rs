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
    let mut head2 = head;
    let mut length = 0;
    let mut elements = vec![];
    while let Some(mut node) = head2 {
        length += 1;
        elements.push(node.val);
        head2 = node.next.take();
    }

    let mut highest = 0;
    let mut i = 0;
    while i < length / 2 {
        let temp_sum = elements[i] + elements[length - 1 - i];
        if temp_sum > highest {
            highest = temp_sum
        }
        i += 1
    }
    highest
}
