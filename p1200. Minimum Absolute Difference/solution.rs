impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let arr = &mut { arr };
        arr.sort();
        let mut min_diff = std::i32::MAX;
        for i in 0..arr.len() - 1 {
            let diff = arr[i + 1] - arr[i];
            if diff < min_diff {
                min_diff = diff;
            }
        }
        let mut pairs = vec![];
        for i in 0..arr.len() - 1 {
            if arr[i + 1] - arr[i] == min_diff {
                pairs.push(vec![arr[i], arr[i + 1]]);
            }
        }
        pairs
    }
}
