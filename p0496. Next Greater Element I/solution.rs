impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut greater_elements = Vec::new();
        'l: for n1 in nums1.into_iter() {
            let mut found = false;
            for n2 in nums2.iter() {
                let n2 = *n2;
                if n2 == n1 {
                    found = true;
                } else if n2 > n1 && found {
                    greater_elements.push(n2);
                    continue 'l;
                }
            }
            greater_elements.push(-1);
        }
        greater_elements
    }
}
