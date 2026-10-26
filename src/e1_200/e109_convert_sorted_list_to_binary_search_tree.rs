use std::rc::Rc;
use std::cell::RefCell;

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

#[inline]
fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for num in nums {
        *tail = Some(Box::new(ListNode::new(num)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None }
    }
}

fn divide_and_conquer(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    let n = nums.len();
    let mut node = TreeNode::new(nums[n/2]);
    if n/2 != 0 {
        node.left = divide_and_conquer(&nums[..n/2]);
    }
    if n/2+1 != n {
        node.right = divide_and_conquer(&nums[n/2+1..]);
    }

    Some(Rc::new(RefCell::new(node)))
}

fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut nums = vec![];
    while head.is_some() {
        nums.push(head.as_ref().unwrap().val);
        head = head.unwrap().next;
    }

    if nums.is_empty() { return None; }
    divide_and_conquer(&nums)
}

pub fn main() {
    let head = [-10,-3,0,5,9].to_vec();
    println!("{:#?}", sorted_list_to_bst(vec_to_list(head)));
}
