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
