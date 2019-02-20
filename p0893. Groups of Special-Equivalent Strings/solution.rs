impl Solution {
    pub fn num_special_equiv_groups(a: Vec<String>) -> i32 {
        let mut groups: Vec<Vec<&String>> = Vec::new();
        for s in &a {
            let mut is_special_equiv = false;
            for group in &mut groups {
                for t in group.iter() {
                    if Solution::is_special_equiv(s, t) {
                        is_special_equiv = true;
                        break;
                    }
                }
                if is_special_equiv {
                    group.push(s);
                    break;
                }
            }
            if !is_special_equiv {
                let mut group = Vec::new();
                group.push(s);
                groups.push(group);
            }
        }
        groups.len() as i32
    }

    fn is_special_equiv(a: &String, b: &String) -> bool {
        Solution::special_sort(a) == Solution::special_sort(b)
    }

    fn special_sort(a: &String) -> String {
        let mut evens = Vec::new();
        let mut odds = Vec::new();
        for (i, c) in a.chars().enumerate() {
            if i % 2 == 0 {
                evens.push(c);
            } else {
                odds.push(c);
            }
        }
        evens.sort();
        odds.sort();
        let mut b = String::with_capacity(a.len());
        for i in 0..a.len() {
            if i % 2 == 0 {
                b.push(evens[i / 2]);
            } else {
                b.push(odds[i / 2]);
            }
        }
        b
    }
}

// fn main() {
//     let a = to_string_vec(vec!["a", "b", "c", "a", "c", "c"]);
//     println!("{:?}", Solution::num_special_equiv_groups(a));
//     let a = to_string_vec(vec!["aa", "bb", "ab", "ba"]);
//     println!("{:?}", Solution::num_special_equiv_groups(a));
//     let a = to_string_vec(vec!["abc", "acb", "bac", "bca", "cab", "cba"]);
//     println!("{:?}", Solution::num_special_equiv_groups(a));
//     let a = to_string_vec(vec!["abcd", "cdab", "adcb", "cbad"]);
//     println!("{:?}", Solution::num_special_equiv_groups(a));
// }

// fn to_string_vec(a: Vec<&str>) -> Vec<String> {
//     let mut b = Vec::<String>::with_capacity(a.len());
//     for s in a {
//         b.push(s.to_string());
//     }
//     b
// }
