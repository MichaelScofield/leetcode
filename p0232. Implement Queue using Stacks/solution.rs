struct MyQueue {
    pub stack: Vec<i32>,
    pub reverse_stack: Vec<i32>,
    pub reverse_flag: bool,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack: Vec::<i32>::new(),
            reverse_stack: Vec::<i32>::new(),
            reverse_flag: false
        }
    }

    fn reverse(&mut self) {
        if self.reverse_flag {
            while let Some(v) = self.reverse_stack.pop() {
                self.stack.push(v);
            }
        } else {
            while let Some(v) = self.stack.pop() {
                self.reverse_stack.push(v);
            }
        }
        self.reverse_flag = !self.reverse_flag;
    }

    fn push(&mut self, x: i32) {
        if self.reverse_flag {
            self.reverse();
        }
        self.stack.push(x)
    }

    fn pop(&mut self) -> i32 {
        if !self.reverse_flag {
            self.reverse();
        }
        self.reverse_stack.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if !self.reverse_flag {
            self.reverse();
        }
        *self.reverse_stack.last().unwrap()
    }

    fn empty(&self) -> bool {
        if self.reverse_flag {
            self.reverse_stack.is_empty()
        } else {
            self.stack.is_empty()
        }
    }
}
