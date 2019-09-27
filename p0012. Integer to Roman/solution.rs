impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut romans = vec![];
        let mut num = num;
        // num is guaranteed to be within the range from 1 to 3999
        (0..num / 1000).for_each(|_x| romans.push('M'));
        num %= 1000;
        for mapping in vec![(100, 'C', 'D', 'M'), (10, 'X', 'L', 'C'), (1, 'I', 'V', 'X')] {
            let d = num / mapping.0;
            match d {
                1 | 2 | 3 => (0..d).for_each(|_x| romans.push(mapping.1)),
                4 => {
                    romans.push(mapping.1);
                    romans.push(mapping.2);
                },
                5 => romans.push(mapping.2),
                6 | 7 | 8 => {
                    romans.push(mapping.2);
                    (0..d - 5).for_each(|_x| romans.push(mapping.1));
                },
                9 => {
                    romans.push(mapping.1);
                    romans.push(mapping.3);
                },
                _ => ()
            }
            num %= mapping.0;
        }
        romans.iter().collect()
    }
}
