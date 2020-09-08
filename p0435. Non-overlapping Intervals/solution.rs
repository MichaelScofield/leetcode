impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let intervals = &mut { intervals };
        let n = intervals.len();
        if n <= 1 {
            return 0;
        }

        use std::cmp::Ordering;
        intervals.sort_by(|a, b| {
            assert!(a.len() == 2 && b.len() == 2 && a[1] > a[0] && b[1] > b[0]);
            match a[1].cmp(&b[1]) {
                Ordering::Equal => a[0].cmp(&b[0]),
                ordering => ordering
            }
        });

        let mut erased = 0;
        let mut i = 0;
        let mut j = 1;
        while j < n {
            if intervals[i][1] <= intervals[j][0] {
                i = j;
            } else {
                erased += 1;
            }
            j += 1;
        }
        erased
    }
}
