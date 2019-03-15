impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let hours = vec![
            vec!["0"], vec!["1", "2", "4", "8"], vec!["3", "5", "6", "9", "10"], vec!["7", "11"]];
        let mut minutes = vec![Vec::<String>::new(); 6];
        let count_set_bits = |x: i32| -> usize {
            let mut x = x;
            let mut c = 0;
            while x > 0 {
                x &= x - 1;
                c += 1;
            }
            c
        };
        for i in 0..60 {
            let set_bits = count_set_bits(i);
            let minutes = &mut minutes[set_bits];
            minutes.push(if i < 10 { format!("0{}", i) } else { format!("{}", i) });
        }

        let num = num as usize;
        let mut times = Vec::new();
        let max_hour_bits = std::cmp::min(num, 3);
        for x in 0..max_hour_bits + 1 {
            let minute_bits = num - x;
            if minute_bits < 6 {
                for hour in &hours[x] {
                    for minute in &minutes[minute_bits] {
                        times.push(format!("{}:{}", hour, minute));
                    }
                }
            }
        }
        times
    }
}
