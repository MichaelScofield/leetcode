impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let alice_candy_sizes = a.iter().sum::<i32>();
        let bob_candy_sizes = b.iter().sum::<i32>();
        for alice_candy_size in a.iter() {
            let alice_candy_sizes = alice_candy_sizes - alice_candy_size;
            let bob_candy_sizes = bob_candy_sizes + alice_candy_size;
            for bob_candy_size in b.iter() {
                if bob_candy_sizes - bob_candy_size == alice_candy_sizes + bob_candy_size {
                    return vec![*alice_candy_size, *bob_candy_size];
                }
            }
        }
        panic!()
    }
}
