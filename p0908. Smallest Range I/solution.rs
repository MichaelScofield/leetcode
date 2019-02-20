impl Solution {
    pub fn smallest_range_i(a: Vec<i32>, k: i32) -> i32 {
        let mut smallest = a[0];
        let mut largest = a[0];
        for x in a {
            if x < smallest {
                smallest = x;
            }
            if x > largest {
                largest = x;
            }
        }
        let gap = largest - smallest - k * 2;
        if gap < 0 { 0 } else { gap }
    }
}
