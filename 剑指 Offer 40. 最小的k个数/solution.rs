impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let len = arr.len();
        if len == 0 || k <= 0 || k > len as i32 {
            return vec![];
        }
        let k = k as usize;
        if k == len {
            return arr;
        }
        fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
            let pivot = arr[low];
            let mut i = low + 1;
            let mut j = high;
            while i <= j {
                if arr[i] > pivot {
                    arr.swap(i, j);
                    j -= 1;
                } else {
                    i += 1;
                }
            }
            arr.swap(low, j);
            j
        }
        let mut low = 0;
        let mut high = len - 1;
        let arr = &mut { arr };
        while low <= high {
            let p = partition(arr, low, high);
            if p == k {
                return arr.as_slice()[0..p].to_vec();
            }
            if p < k {
                low = p + 1;
            } else {
                high = p - 1;
            }
        }
        vec![]
    }
}
