use std::collections::VecDeque;

struct RecentCounter {
    pings: VecDeque<i32>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            pings: VecDeque::new()
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        let front = match self.pings.front() {
            Some(value) => *value,
            None => 0
        };

        if front > 0 && t - front > 3000 {
            self.pings.pop_front();
        }
        self.pings.push_back(t);

        let mut count = 0;
        for ping in self.pings.iter() {
            if t - ping <= 3000 {
                count += 1;
            }
        }
        count
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
