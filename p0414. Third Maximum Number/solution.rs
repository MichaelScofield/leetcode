impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut max_3 = Vec::with_capacity(3);
        'l: for num in nums {
            let l = max_3.len();
            if l < 3 {
                for i in 0..l {
                    if max_3[i] == num {
                        continue 'l;
                    }
                }
                max_3.push(num);
                max_3.sort_by(|a, b| b.cmp(a));
            } else {
                if num > max_3[2] {
                    for i in 0..3 {
                        if num == max_3[i] {
                            break;
                        }
                        if num > max_3[i] {
                            let mut j = 2;
                            while j > i {
                                max_3[j] = max_3[j - 1];
                                j -= 1;
                            }
                            max_3[i] = num;
                            break;
                        }
                    }
                }
            }
        }
        if max_3.len() < 3 { max_3[0] } else { max_3[2] }
    }
}
