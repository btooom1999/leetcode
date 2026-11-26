use rand::Rng;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
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

#[derive(Debug)]
struct Solution {
    head: Option<Box<ListNode>>,
}

impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }

    fn get_random(&mut self) -> i32 {
        let mut head = self.head.as_mut();
        let mut chosen = 0;
        let mut count = 0;

        while let Some(node) = head {
            count += 1;

            if rand::random_range(0..count) == 0 {
                chosen = node.val;
            }

            head = node.next.as_mut();
        }

        chosen
    }
}

pub fn main() {
    let mut solution = Solution::new(vec_to_list(vec![1]));
    println!("{:?}", solution.get_random());
    println!("{:?}", solution.get_random());
    println!("{:?}", solution.get_random());
    println!("{:?}", solution.get_random());
    println!("{:?}", solution.get_random());
    println!("{:?}", solution.get_random());
}
