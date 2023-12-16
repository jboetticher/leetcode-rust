use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }
        let (mut s_map, mut t_map) = (HashMap::new(), HashMap::new());
        Self::build_hashmap(s, &mut s_map);
        Self::build_hashmap(t, &mut t_map);
        s_map.iter().all(|(c, n)| t_map.get(c) == Some(n))
    }

    fn build_hashmap(s: String, h: &mut HashMap<char, i32>) {
        for c in s.chars() {
            h.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
    }
}
