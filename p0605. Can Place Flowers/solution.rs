impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut i = 0;
        let l = flowerbed.len();
        while i < l && n > 0 {
            if flowerbed[i] == 1 {
                i += 2;
            } else {
                if i > 0 && flowerbed[i - 1] == 1 {
                    i += 1;
                    continue;
                }
                if i < l - 1 && flowerbed[i + 1] == 1 {
                    i += 3;
                    continue;
                }
                n -= 1;
                i += 2;
            }
        }
        n == 0
    }
}
