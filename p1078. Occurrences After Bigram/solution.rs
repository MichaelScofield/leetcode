impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut answer = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut i = 0;
        loop {
            if i + 2 >= words.len() {
                break;
            }
            if words[i] == first && words[i + 1] == second {
                answer.push(words[i + 2].to_string());
            }
            i += 1;
        }
        answer
    }
}
