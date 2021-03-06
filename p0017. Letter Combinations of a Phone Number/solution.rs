impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let l = digits.len();
        if l == 0 {
            return Vec::with_capacity(0);
        }
        let letters_mapping = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
            vec!['j', 'k', 'l'],
            vec!['m', 'n', 'o'],
            vec!['p', 'q', 'r', 's'],
            vec!['t', 'u', 'v'],
            vec!['w', 'x', 'y', 'z']
        ];
        let capacity: usize = digits.as_bytes().iter()
            .map(|&digit| letters_mapping[(digit - b'2') as usize].len())
            .product();
        let mut combinations = Vec::with_capacity(capacity);
        fn letter_combinations(digits: &[u8], i: usize, combination: &mut [char],
                               combinations: &mut Vec<String>, letters_mapping: &Vec<Vec<char>>) {
            let l = digits.len();
            for &c in letters_mapping[(digits[i] - b'2') as usize].iter() {
                combination[i] = c;
                if i == l - 1 {
                    combinations.push(combination.iter().collect());
                } else {
                    letter_combinations(digits, i + 1, combination, combinations, letters_mapping);
                }
            }
        }
        let digits = digits.as_bytes();
        let combination = &mut vec!['0'; l];
        letter_combinations(digits, 0, combination, &mut combinations, &letters_mapping);
        combinations
    }
}
