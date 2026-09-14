use std::rc::Rc;
use std::cell::RefCell;

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

struct Codec {
    data: Vec<i32>,
    i: usize,
}

impl Codec {
    fn new() -> Self {
        Self { data: vec![], i: 0 }
    }

    fn build(&mut self, node_rc: Rc<RefCell<TreeNode>>) {
        let node = node_rc.borrow();
        self.data.push(node.val);
        if node.left.is_none() {
            self.data.push(-1001);
        } else {
            self.build(node.left.clone().unwrap());
        }

        if node.right.is_none() {
            self.data.push(-1001);
        } else {
            self.build(node.right.clone().unwrap());
        }
    }

    fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "".to_string();
        }

        self.build(root.clone().unwrap());
        let res = self.data.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");
        self.data = vec![];
        res
    }

    fn convert(&mut self, data: &[&str]) -> Option<Rc<RefCell<TreeNode>>> {
        let root = Rc::new(RefCell::new(TreeNode::new(data[self.i].parse::<i32>().unwrap())));
        self.i += 1;

        if data[self.i] != "-1001" {
            root.borrow_mut().left = self.convert(data);
        } else {
            self.i += 1;
        }

        if data[self.i] != "-1001" {
            root.borrow_mut().right = self.convert(data);
        } else {
            self.i += 1;
        }

        Some(root)
    }

    fn deserialize(&mut self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }

        self.convert(&data.split(',').collect::<Vec<_>>())
    }
}

pub fn main() {

}
