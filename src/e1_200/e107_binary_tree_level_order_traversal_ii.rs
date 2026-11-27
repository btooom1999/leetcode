use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

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

fn vec_to_list(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap())));
    let mut nodes = vec![root.clone()];

    let mut i = 1;
    let n = nums.len();
    while i < n {
        let mut new_nodes = vec![];
        for node_rc in &nodes {
            let mut node = node_rc.borrow_mut();
            if i < n && let Some(num) = nums[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(num)));
                node.left = Some(left.clone());
                new_nodes.push(left);
            }

            i += 1;

            if i < n && let Some(num) = nums[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(num)));
                node.right = Some(right.clone());
                new_nodes.push(right);
            }

            i += 1;
        }

        nodes = new_nodes;
    }

    Some(root)
}

fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut ans = vec![];
    let mut q = VecDeque::from([(root.unwrap(), 0)]);
    while let Some((node_rc, i)) = q.pop_front() {
        let node = node_rc.borrow();
        if ans.len() == i {
            ans.push(vec![]);
        }

        ans[i].push(node.val);

        if let Some(child_rc) = node.left.clone() {
            q.push_back((child_rc.clone(), i+1));
        }

        if let Some(child_rc) = node.right.clone() {
            q.push_back((child_rc.clone(), i+1));
        }
    }

    ans.reverse();
    ans
}

pub fn main() {
    let root = vec_to_list(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    println!("{:?}", level_order_bottom(root));
}
