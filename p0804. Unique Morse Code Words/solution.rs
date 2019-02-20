impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let morse_codes = [
            ".-", "-...", "-.-.", "-..", ".",
            "..-.", "--.", "....", "..", ".---",
            "-.-", ".-..", "--", "-.", "---",
            ".--.", "--.-", ".-.", "...", "-",
            "..-", "...-", ".--", "-..-", "-.--",
            "--.."];

        let mut morse_strings = HashSet::new();
        let zero = 'a' as u32;
        for word in words {
            let mut morse_string = String::new();
            for c in word.chars() {
                let offset = c as u32 - zero;
                let morse_code = morse_codes[offset as usize];
                morse_string.push_str(morse_code);
            }
            morse_strings.insert(morse_string);
        }
        return morse_strings.len() as i32;
    }
}
