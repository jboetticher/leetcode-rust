use std::collections::HashSet;
impl Solution {
    pub fn close_strings(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }

        let mut w1_char_count: [i32; 26] = [0; 26];
        let mut w2_char_count: [i32; 26] = [0; 26];
        let mut w1_set = HashSet::new();
        let mut w2_set = HashSet::new();

        s.as_bytes().iter().zip(t.as_bytes().iter()).for_each(|c| {
            w1_char_count[(c.0 - 'a' as u8) as usize] += 1;
            w1_set.insert(c.0);
            w2_char_count[(c.1 - 'a' as u8) as usize] += 1;
            w2_set.insert(c.1);
        });

        if w1_set.difference(&w2_set).next() != None { return false; }

        w1_char_count.sort_unstable();
        w2_char_count.sort_unstable();
        w1_char_count.iter().zip(w2_char_count.iter()).all(|x| x.0 == x.1)
    }
}

