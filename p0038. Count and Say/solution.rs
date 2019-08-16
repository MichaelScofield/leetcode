impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut say = "1".chars().collect::<Vec<char>>();
        for _ in 1..n {
            let mut last_char = say[0];
            let mut count = 1;
            let mut next_say = vec![];
            for i in 1..say.len() {
                if say[i] == last_char {
                    count += 1;
                } else {
                    next_say.push(std::char::from_digit(count, 10).unwrap());
                    next_say.push(last_char);
                    last_char = say[i];
                    count = 1;
                }
            }
            next_say.push(std::char::from_digit(count, 10).unwrap());
            next_say.push(last_char);
            say = next_say;
        }
        say.iter().collect::<String>()
    }
}
