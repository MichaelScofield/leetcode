impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut b = a.clone();
        let mut i = 0;
        let mut j = b.len() - 1;
        while i < j {
            if b[i] % 2 == 1 {
                b.swap(i, j);
                j -= 1;
            } else {
                i += 1;
            }
        }
        return b;
    }
}
