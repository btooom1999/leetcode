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

fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut odd_head = None;
    let mut odd_tail = &mut odd_head;

    let mut even_head = None;
    let mut even_tail = &mut even_head;

    let mut is_odd = true;
    while let Some(mut node) = head {
        head = node.next.take();
        if !is_odd {
            *even_tail = Some(node);
            even_tail = &mut even_tail.as_mut().unwrap().next;
        } else {
            *odd_tail = Some(node);
            odd_tail = &mut odd_tail.as_mut().unwrap().next;
        }

        is_odd = !is_odd;
    }

    *odd_tail = even_head;
    odd_head
}

pub fn main() {
    let head = vec_to_list(vec![1,2,3,4,5]);
    println!("{:?}", odd_even_list(head));
}
