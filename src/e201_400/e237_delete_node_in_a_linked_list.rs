#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for num in nums {
        *tail = Some(Box::new(ListNode::new(num)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

fn delete_node(node: &mut Option<Box<ListNode>>) {
    if let Some(node) = node.as_mut() && node.next.is_some() {
        node.val = node.next.as_ref().unwrap().val;
        node.next = node.next.as_mut().unwrap().next.take();
    }
}

pub fn main() {}
