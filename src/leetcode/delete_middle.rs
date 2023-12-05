#[derive(PartialEq, Eq, Clone, Debug)]
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

pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut position = 0;

    let mut node = head.as_ref();
    while node.is_some() {
        node = node.unwrap().next.as_ref();
        position += 1;
    }

    if position <= 1 {
        return Option::None;
    }

    position /= 2;
    let mut node = head.as_mut();
    while position > 1 {
        node = node.unwrap().next.as_mut();
        position -= 1;
    }
    node.unwrap().next = node.as_mut().unwrap().next.as_mut().unwrap().next.take();

    return head;
}
