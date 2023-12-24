use std::cmp::min;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        // There are only two possible situations. 010101... and 101010...
        // Just find the minimum out of both
        let (mut leading_zero, mut leading_one) = (0, 0);
        let mut place = 0;
        for c in s.chars() {
            let even_place = place % 2 == 0;
            let is_zero = c == '0';
            if is_zero && even_place || !is_zero && !even_place {
                leading_one += 1;
            }
            else if is_zero && !even_place || !is_zero && even_place {
                leading_zero += 1;
            }
            place += 1;
        }
        min(leading_zero, leading_one)
    }
}
