impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        stones.iter().for_each(|&x| heap.push(x));
        loop {
            let stone1 = heap.pop();
            if let Some(stone1) = stone1 {
                let stone2 = heap.pop();
                if let Some(stone2) = stone2 {
                    let smash: i32 = (stone1 - stone2).abs();
                    if smash > 0 {
                        heap.push(smash);
                    }
                } else {
                    return stone1;
                }
            } else {
                break;
            }
        }
        0
    }
}
