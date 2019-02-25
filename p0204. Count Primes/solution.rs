impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut primes = 0;
        for i in 2..n {
            if i == 2 {
                primes += 1;
                continue;
            }
            if i % 2 == 0 {
                continue;
            }
            let mut is_prime = true;
            let s = (i as f64).sqrt() as i32;
            for j in 2..s + 1 {
                if i % j == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                primes += 1;
            }
        }
        primes
    }
}
