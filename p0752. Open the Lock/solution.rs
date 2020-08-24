impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        if target.eq("0000") {
            return 0;
        }
        use std::collections::HashSet;
        let deadends = deadends.into_iter()
            .map(|deadend| deadend.into_bytes().into_iter()
                .map(|b| b - 48)
                .collect::<Vec<u8>>())
            .collect::<HashSet<Vec<u8>>>();
        if deadends.contains(&vec![0, 0, 0, 0]) {
            return -1;
        }
        let target = target.into_bytes().into_iter().map(|b| b - 48).collect::<Vec<u8>>();
        let mut tried_locks = HashSet::new();
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        queue.push_back((vec![0, 0, 0, 0], 0));
        while let Some((mut lock, turns)) = queue.pop_front() {
            if tried_locks.contains(&lock) {
                continue;
            }
            tried_locks.insert(lock.clone());
            if lock.eq(&target) {
                return turns;
            }
            for i in 0..4 {
                let stash = lock[i];
                for j in vec![1, 9] {
                    lock[i] = (stash + j) % 10;
                    if !deadends.contains(&lock) {
                        queue.push_back((lock.clone(), turns + 1));
                    }
                }
                lock[i] = stash;
            }
        }
        -1
    }
}
