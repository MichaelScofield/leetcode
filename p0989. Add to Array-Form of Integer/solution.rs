impl Solution {
    pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
        let mut b = Vec::with_capacity(a.len() + 1);
        let mut i = a.len() as i32 - 1;
        let mut k = k;
        let mut carry = 0;
        while i >= 0 || k > 0 {
            let r = k % 10;
            let add = if i >= 0 { a[i as usize] } else { 0 } + r + carry;
            b.push(add % 10);
            i -= 1;
            k /= 10;
            carry = if add > 9 { 1 } else { 0 };
        }
        if carry == 1 {
            b.push(1);
        }
        b.reverse();
        b
    }
}
