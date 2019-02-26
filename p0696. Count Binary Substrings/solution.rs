impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut count = 0;
        let chars: Vec<char> = s.chars().collect();
        let l = chars.len();
        let mut i;
        let mut switching_index = 0; // index that 0 switches to 1 or vice versa
        let mut last_switching_index;
        let mut stack = Vec::with_capacity(chars.len());
        loop {
            last_switching_index = switching_index;
            i = switching_index;
            let mut expect = None;
            while i < l {
                let c = chars[i];
                if expect != None && c != expect.unwrap() {
                    if switching_index == last_switching_index {
                        switching_index = i;
                    } else { // greedy ends
                        break;
                    }
                }
                expect = Some(c);
                stack.push(c);
                i += 1;
            }
            if last_switching_index != switching_index {
                let delta = switching_index - last_switching_index;
                count += std::cmp::min(delta, stack.len() - delta);
            } else {
                break;
            }
            stack.clear();
        }
        count as i32
    }
}
