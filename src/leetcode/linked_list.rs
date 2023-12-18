pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut head = head;
    let mut current = head.as_mut();

    while let Some(current_inner) = current {
        while let Some(next_inner) = current_inner.next.as_mut() {
            if next_inner.val == current_inner.val {
                current_inner.next = next_inner.next.take();
            } else {
                break;
            }
        }
        current = current_inner.next.as_mut();
    }

    head
}

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
