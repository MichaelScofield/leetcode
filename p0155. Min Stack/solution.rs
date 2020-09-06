struct MinStack {
    data_stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            data_stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.data_stack.push(x);
        if let Some(&min) = self.min_stack.last() {
            if min < x {
                self.min_stack.push(min);
                return;
            }
        }
        self.min_stack.push(x);
    }

    fn pop(&mut self) {
        self.data_stack.pop();
        self.min_stack.pop();
    }

    fn top(&mut self) -> i32 {
        if let Some(&x) = self.data_stack.last() {
            return x;
        }
        panic!("Illegal State!")
    }

    fn get_min(&self) -> i32 {
        if let Some(&min) = self.min_stack.last() {
            return min;
        }
        panic!("Illegal State!")
    }
}
