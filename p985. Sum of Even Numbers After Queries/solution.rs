impl Solution {
    pub fn sum_even_after_queries(a: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let a = &mut { a };
        let mut b = Vec::with_capacity(queries.len());
        for query in queries {
            let val = query[0];
            let index = query[1] as usize;
            a[index] += val;
            let mut x = 0;
            for i in a.iter() {
                if *i % 2 == 0 {
                    x += i;
                }
            }
            b.push(x);
        }
        b
    }
}
