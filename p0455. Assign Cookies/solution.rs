impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let g = &mut { g };
        g.sort();
        let s = &mut { s };
        s.sort();
        let mut i = 0;
        let mut j = 0;
        let mut content_children = 0;
        while i < g.len() && j < s.len() {
            if g[i] <= s[j] {
                content_children += 1;
                i += 1;
            }
            j += 1;
        }
        content_children
    }
}
