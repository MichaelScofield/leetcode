impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        match num {
            1 => false,
            _ => {
                let mut divisors = vec![1];
                for i in 2..(num as f64).sqrt() as i32 + 1 {
                    if num % i == 0 {
                        divisors.push(i);
                        divisors.push(num / i);
                    }
                }
                divisors.iter().sum::<i32>() == num
            }
        }
    }
}
