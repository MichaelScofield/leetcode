impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut e = 0; // first even index that a[e] is not even
        let mut found_e = false;
        let mut o = 1; // first odd index that a[o] is not odd
        let mut found_o = false;

        let mut b = a.clone();
        let l = b.len();
        loop {
            if e < l {
                if a[e] % 2 == 0 {
                    e += 2;
                } else {
                    found_e = true;
                }
            }

            if o < l {
                if a[o] % 2 != 0 {
                    o += 2;
                } else {
                    found_o = true;
                }
            }

            if found_e && found_o {
                b.swap(e, o);

                e += 2;
                found_e = false;
                o += 2;
                found_o = false;
            }
            if e >= l && o >= l {
                break;
            }
        }
        b
    }
}
