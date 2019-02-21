impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let (x1, y1) = (rec1[0], rec1[1]);
        let (x2, y2) = (rec1[2], rec1[3]);
        let (a1, b1) = (rec2[0], rec2[1]);
        let (a2, b2) = (rec2[2], rec2[3]);
        !(a1 >= x2 || a2 <= x1 || b1 >= y2 || b2 <= y1)
    }
}
