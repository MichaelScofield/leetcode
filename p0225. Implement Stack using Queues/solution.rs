use std::collections::VecDeque;

struct MyStack {
    main_queue: VecDeque<i32>,
    standby_queue: VecDeque<i32>,
    main_active: bool
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            main_queue: VecDeque::<i32>::new(),
            standby_queue: VecDeque::<i32>::new(),
            main_active: true
        }
    }

    fn push(&mut self, x: i32) {
        if self.main_active {
            self.main_queue.push_back(x);
        } else {
            self.standby_queue.push_back(x);
        }
    }

    fn pop(&mut self) -> i32 {
        let mut pop = None;
        if self.main_active {
            loop {
                if let Some(v) = self.main_queue.pop_front() {
                    if self.main_queue.is_empty() {
                        pop = Some(v);
                        break;
                    }
                    self.standby_queue.push_back(v);
                } else {
                    break;
                }
            }
        } else {
            loop {
                if let Some(v) = self.standby_queue.pop_front() {
                    if self.standby_queue.is_empty() {
                        pop = Some(v);
                        break;
                    }
                    self.main_queue.push_back(v);
                } else {
                    break;
                }
            }
        }
        if let Some(v) = pop {
            self.main_active = !self.main_active;
            v
        } else {
            panic!("not valid operation")
        }
    }

    fn top(&mut self) -> i32 {
        let mut top = None;
        if self.main_active {
            loop {
                if let Some(v) = self.main_queue.pop_front() {
                    if self.main_queue.is_empty() {
                        top = Some(v);
                    }
                    self.standby_queue.push_back(v);
                } else {
                    break;
                }
            }
        } else {
            loop {
                if let Some(v) = self.standby_queue.pop_front() {
                    if self.standby_queue.is_empty() {
                        top = Some(v);
                    }
                    self.main_queue.push_back(v);
                } else {
                    break;
                }
            }
        }
        if let Some(v) = top {
            self.main_active = !self.main_active;
            v
        } else {
            panic!("not valid operation")
        }
    }

    fn empty(&self) -> bool {
        if self.main_active {
            self.main_queue.is_empty()
        } else {
            self.standby_queue.is_empty()
        }
    }
}
