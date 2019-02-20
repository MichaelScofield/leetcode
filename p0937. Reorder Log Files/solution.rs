impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut reordered_logs = Vec::with_capacity(logs.len());
        let mut digit_logs = Vec::new();
        let mut letter_logs = Vec::new();
        for log in logs.iter() {
            let mut whitespace_index;
            let mut is_digit_log = false;
            let mut chars = log.chars().enumerate();
            loop {
                let (i, c) = chars.next().unwrap();
                if c == ' ' {
                    whitespace_index = i;
                    if chars.next().unwrap().1.is_digit(10) {
                        is_digit_log = true;
                    }
                    break;
                }
            }
            if is_digit_log {
                digit_logs.push(log.clone());
            } else {
                letter_logs.push(log.split_at(whitespace_index));
            }
        }

        letter_logs.sort_by_key(|(_identifier, words)| *words);
        let mut letter_logs: Vec<String> = letter_logs.into_iter()
            .map(|(identifier, words)| format!("{}{}", identifier, words)).collect();

        reordered_logs.append(&mut letter_logs);
        reordered_logs.append(&mut digit_logs);
        reordered_logs
    }
}
