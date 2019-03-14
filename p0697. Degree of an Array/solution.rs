impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut frequencies = HashMap::new();
        for &num in nums.iter() {
            let frequency = frequencies.entry(num).or_insert(0);
            *frequency += 1;
        }
        let mut max_frequency = 0;
        for (_, &frequency) in frequencies.iter() {
            if frequency > max_frequency {
                max_frequency = frequency;
            }
        }
        if max_frequency == 1 {
            return 1;
        }
        let max_frequency_nums = frequencies.iter()
            .filter(|&(_, &frequency)| frequency == max_frequency)
            .map(|(&num, _)| num).collect::<Vec<i32>>();

        let mut shortest_sub_array_len: usize = std::usize::MAX;
        for max_frequency_num in max_frequency_nums {
            let mut frequency = max_frequency;
            let mut start = None;
            let mut sub_array_len = 0;
            for (i, &num) in nums.iter().enumerate() {
                if num == max_frequency_num {
                    frequency -= 1;
                    if frequency == 0 {
                        sub_array_len = i - start.unwrap() + 1;
                        break;
                    }
                    if start == None {
                        start = Some(i);
                    }
                }
            }
            if frequency == 0 && sub_array_len < shortest_sub_array_len {
                shortest_sub_array_len = sub_array_len;
            }
        }
        shortest_sub_array_len as i32
    }
}
