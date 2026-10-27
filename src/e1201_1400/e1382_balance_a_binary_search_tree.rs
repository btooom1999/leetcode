use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None }
    }
}

fn vec_to_tree(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap())));
    let mut nodes = vec![root.clone()];

    let mut i = 1;
    while i < nums.len() {
        let mut new_nodes = Vec::new();
        for node_rc in nodes.iter() {
            let mut node = node_rc.borrow_mut();

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(left.clone());
                new_nodes.push(left.clone());
            }
            i += 1;

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let right= Rc::new(RefCell::new(TreeNode::new(val)));
                node.right = Some(right.clone());
                new_nodes.push(right.clone());
            }
            i += 1;
        }

        nodes = new_nodes
    }

    Some(root)
}

fn dfs(
    root: Option<Rc<RefCell<TreeNode>>>,
    sorted_nums: &mut Vec<i32>,
) {
    if let Some(node_rc) = root {
        let node = node_rc.borrow();
        dfs(node.left.clone(), sorted_nums);
        sorted_nums.push(node.val);
        dfs(node.right.clone(), sorted_nums);
    }
}

fn divide_and_conquer(sorted_nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if sorted_nums.is_empty() { return None; }
    let m = sorted_nums.len()/2;
    let mut node = TreeNode::new(sorted_nums[m]);
    node.left = divide_and_conquer(&sorted_nums[..m]);
    node.right = divide_and_conquer(&sorted_nums[m+1..]);

    Some(Rc::new(RefCell::new(node)))
}

fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut sorted_nums = vec![];
    dfs(root, &mut sorted_nums);

    divide_and_conquer(&sorted_nums)
}

pub fn main() {
    let root = vec_to_tree(vec![Some(1), None, Some(2), None, Some(3), None, Some(4), None, None]);
    println!("{:#?}", balance_bst(root));
}
