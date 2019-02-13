impl Solution {
    pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        let mut primes = 0;
        for i in l..r + 1 {
            let mut set_bits = 0;
            let mut j = i;
            while j > 0 {
                if j % 2 == 1 {
                    set_bits += 1;
                }
                j /= 2;
            }
            if Self::is_prime(set_bits) {
                primes += 1;
            }
        }
        primes
    }

    fn is_prime(n: i32) -> bool {
        if n == 1 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        for i in (3..(n as f64).sqrt() as i32 + 1).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}
