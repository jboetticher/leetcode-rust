use std::cmp;

impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut counter = 0;
        let mut flips = 0;
        for c in s.chars() {
            if c == '1' {
                counter += 1;
            }
            else {
                flips += 1;
            }
            flips = cmp::min(flips, counter);
        }

        flips
    }
}
