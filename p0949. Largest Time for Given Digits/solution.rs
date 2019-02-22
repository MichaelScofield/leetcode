impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        let a = &mut { a };
        let mut times = Vec::new();
        for i in 0..a.len() {
            if a[i] > 2 {
                continue;
            }
            for j in 0..a.len() {
                if j == i {
                    continue;
                }
                let hour = a[i] * 10 + a[j];
                if hour < 24 {
                    for x in 0..a.len() {
                        if x == i || x == j {
                            continue;
                        }
                        for y in 0..a.len() {
                            if y == x || y == i || y == j {
                                continue;
                            }
                            let minute = a[x] * 10 + a[y];
                            if minute < 60 {
                                times.push(hour * 100 + minute);
                            }
                        }
                    }
                }
            }
        }
        match times.iter().max() {
            None => "".to_string(),
            Some(time) => {
                let mut largest_time = String::with_capacity(5);
                if *time < 10 {
                    largest_time.push_str("000");
                } else if *time < 100 {
                    largest_time.push_str("00");
                } else if *time < 1000 {
                    largest_time.push_str("0");
                }
                largest_time.push_str(time.to_string().as_str());
                largest_time.insert(2, ':');
                largest_time
            }
        }
    }
}
