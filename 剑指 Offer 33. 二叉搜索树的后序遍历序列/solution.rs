impl Solution {
    pub fn verify_postorder(postorder: Vec<i32>) -> bool {
        if postorder.len() < 1 {
            return true;
        }
        fn verify_postorder(postorder: &[i32]) -> bool {
            let n = postorder.len();
            if n <= 1 {
                return true;
            }
            let root = postorder[n - 1];
            let mut i = 0;
            while i < n {
                if postorder[i] >= root {
                    break;
                }
                i += 1;
            }
            for j in i + 1..n {
                if postorder[j] < root {
                    return false;
                }
            }
            let mut verify_left = true;
            if i > 0 {
                verify_left = verify_postorder(&postorder[0..i]);
            }
            let mut verify_right = true;
            if i < n {
                verify_right = verify_postorder(&postorder[i..n - 1]);
            }
            verify_left && verify_right
        }
        verify_postorder(postorder.as_slice())
    }
}
