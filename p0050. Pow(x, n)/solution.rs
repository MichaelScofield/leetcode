impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if x == 0.0 {
            assert!(n > 0);
            return 0.0;
        }
        let m = n.abs() as u32;
        use std::collections::HashMap;
        let mut memo = HashMap::new();
        fn pow(x: f64, i: u32, memo: &mut HashMap<u32, f64>) -> f64 {
            if i == 0 {
                return 1.0;
            }
            if i == 1 {
                return x;
            }
            if let Some(&value) = memo.get(&i) {
                return value;
            }
            let a = pow(x, i / 2, memo);
            memo.insert(i / 2, a);
            if i % 2 == 0 {
                a * a
            } else {
                memo.insert(i / 2 + 1, a * x);
                a * a * x
            }
        }
        let result = pow(x, m, &mut memo);
        if n > 0 { result } else { 1.0 / result }
    }
}
