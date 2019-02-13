impl Solution {
    pub fn to_goat_latin(s: String) -> String {
        let vowels = vec!['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

        let mut goat_latin_words = Vec::new();
        for (i, word) in s.split_whitespace().enumerate() {
            let mut goat_latin_word = String::new();

            let first_char = word.chars().next().unwrap();
            match vowels.binary_search(&first_char) {
                Ok(_) => goat_latin_word.push_str(word),
                Err(_) => {
                    goat_latin_word.push_str(&word[1..]);
                    goat_latin_word.push_str(&word[0..1])
                }
            }
            goat_latin_word.push_str("ma");

            for _j in 0..i + 1 {
                goat_latin_word.push('a');
            }

            goat_latin_words.push(goat_latin_word);
        }
        goat_latin_words.join(" ")
    }
}
