#[derive(Debug, Clone)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

struct MyCircularDeque {
    capacity: i32,
    size: i32,
    head: Option<Box<ListNode>>,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self { capacity: k, size: 0, head: None }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let mut node = ListNode::new(value);
        node.next = self.head.clone();
        self.head = Some(Box::new(node));
        self.size += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        let node = ListNode::new(value);
        let mut tail = &mut self.head;
        while tail.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }

        *tail = Some(Box::new(node));
        self.size += 1;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.head = self.head.as_mut().unwrap().next.take();
        self.size -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        let mut head = &mut self.head;
        while head.as_ref().unwrap().next.is_some() {
            head = &mut head.as_mut().unwrap().next;
        }

        *head = None;
        self.size -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        self.head.as_ref().unwrap().val
    }

    fn get_rear(&mut self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        let mut head = &mut self.head;
        while head.as_ref().unwrap().next.is_some() {
            head = &mut head.as_mut().unwrap().next;
        }

        head.as_ref().unwrap().val
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

pub fn main() {}
