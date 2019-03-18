impl Solution {
    pub fn largest_sum_after_k_negations(a: Vec<i32>, k: i32) -> i32 {
        let a = &mut { a };
        a.sort();
        let mut k = k;
        for x in a.iter_mut() {
            if k <= 0 {
                break;
            }
            if *x < 0 {
                *x = -*x;
                k -= 1;
            } else if *x == 0 {
                k = 0;
                break;
            }
        }
        if k > 0 {
            if k % 2 == 1 {
                a.sort();
                a[0] = -a[0];
            }
        }
        a.iter().sum()
    }
}
