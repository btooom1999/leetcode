struct MyQueue {
    stack: [Vec<i32>; 2],
}

impl MyQueue {
    fn new() -> Self {
        const VEC: Vec<i32> = Vec::new();
        Self { stack: [VEC; 2] }
    }

    fn push(&mut self, x: i32) {
        self.stack[0].push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.stack[1].is_empty() {
            while let Some(num) = self.stack[0].pop() {
                self.stack[1].push(num);
            }
        }

        self.stack[1].pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.stack[1].is_empty() {
            while let Some(num) = self.stack[0].pop() {
                self.stack[1].push(num);
            }
        }

        *self.stack[1].last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack[1].is_empty()
    }
}

pub fn main() {

}
