impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut change = (0, 0, 0);
        for bill in bills {
            if bill == 5 {
                change.0 += 1;
            } else if bill == 10 {
                if change.0 < 1 {
                    return false;
                }
                change.0 -= 1;
                change.1 += 1;
            } else if bill == 20 {
                if change.0 > 0 && change.1 > 0 {
                    change.0 -= 1;
                    change.1 -= 1;
                } else if change.0 > 2 {
                    change.0 -= 3;
                } else {
                    return false;
                }
            }
        }
        true
    }
}
