impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut char_count: [(i32, i32); 26] = [(0, 0); 26];
        s.as_bytes().iter().zip(t.as_bytes().iter()).for_each(|c| {
            char_count[(c.0 - 'a' as u8) as usize].0 += 1;
            char_count[(c.1 - 'a' as u8) as usize].1 += 1;
        });
        (char_count.iter().fold(0, |acc, x| acc + (x.0 - x.1).abs()) + 1) / 2
    }
}
