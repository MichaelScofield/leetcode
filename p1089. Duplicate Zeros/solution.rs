impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let l = arr.len();
        let mut i = 0;
        while i < l {
            if arr[i] == 0 {
                arr.insert(i + 1, 0);
                arr.remove(l);
                i += 1;
            }
            i += 1;
        }
    }
}
