impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        use std::collections::HashSet;
        let mut powerful_integers = HashSet::new();
        let mut i = 0;
        let mut last_xp = -1;
        loop {
            let xp = x.pow(i);
            if xp >= bound || xp == last_xp {
                break;
            }
            last_xp = xp;

            let mut j = 0;
            let mut last_yp = -1;
            loop {
                let yp = y.pow(j);
                if xp + yp > bound || yp == last_yp {
                    break;
                } else {
                    powerful_integers.insert(xp + yp);
                    last_yp = yp;
                }
                j += 1;
            }
            i += 1;
        }
        powerful_integers.into_iter().collect::<Vec<i32>>()
    }
}
