use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
struct FreqStack {
    hashmap: HashMap<i32, Vec<usize>>,
    btreemap: BTreeMap<(usize, usize), i32>,
    len: usize,
}

impl FreqStack {
    fn new() -> Self {
        Self { hashmap: HashMap::new(), btreemap: BTreeMap::new(), len: 0, }
    }

    fn push(&mut self, val: i32) {
        let vec= self.hashmap.entry(val).or_default();
        let k = (vec.len(), *vec.last().unwrap_or(&usize::MAX));
        self.btreemap.remove(&k);

        vec.push(self.len);
        self.btreemap.insert((vec.len(), self.len), val);
        self.len += 1;
    }

    fn pop(&mut self) -> i32 {
        if self.hashmap.is_empty() {
            return -1;
        }

        let val = self.btreemap.pop_last().unwrap().1;
        if let Some(vec) = self.hashmap.get_mut(&val) {
            vec.pop();
            if vec.is_empty() {
                self.hashmap.remove(&val);
            } else {
                self.btreemap.insert((vec.len(), *vec.last().unwrap()), val);
            }
        }

        val
    }
}

pub fn main() {
    let mut freq_stack = FreqStack::new();
    freq_stack.push(5);
    freq_stack.push(7);
    freq_stack.push(5);
    freq_stack.push(7);
    freq_stack.push(4);
    freq_stack.push(5);
    println!("{}", freq_stack.pop());
    println!("{}", freq_stack.pop());
    println!("{}", freq_stack.pop());
    println!("{}", freq_stack.pop());
    println!("{:?}", freq_stack.btreemap);
}
