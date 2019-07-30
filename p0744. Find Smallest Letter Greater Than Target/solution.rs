impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let len = letters.len();
        let mut i = 0;
        let mut j = len;
        while i < j {
            let mid = i + (j - i) / 2;
            if letters[mid] < target {
                i = mid + 1;
            } else if letters[mid] == target {
                let mut k = mid + 1;
                while k < len {
                    if letters[k] > target {
                        return letters[k];
                    }
                    k += 1;
                }
                if k == len {
                    return letters[0];
                }
            } else {
                j = mid;
            }
        }
        letters[i % len]
    }
}
