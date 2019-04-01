impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let sum: i32 = a.iter().sum();
        if sum % 3 != 0 {
            return false;
        }
        let expect = sum / 3;
        let l = a.len();
        let mut sum1 = 0;
        let mut sum2;
        for (i, &x) in a.iter().enumerate() {
            sum1 += x;
            if sum1 == expect {
                sum2 = 0;
                for j in i + 1..l {
                    sum2 += a[j];
                    if sum2 == expect && j < l - 1 {
                        return true;
                    }
                }
            }
        }
        false
    }
}
