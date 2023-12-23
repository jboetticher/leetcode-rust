use std::collections::HashSet;
impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut h = HashSet::<(i32, i32)>::from([(0, 0)]);
        let mut cur = (0, 0);
        for c in path.chars() {
            match c {
                'N' => cur = (cur.0, cur.1 + 1),
                'S' => cur = (cur.0, cur.1 - 1),
                'E' => cur = (cur.0 + 1, cur.1),
                'W' => cur = (cur.0 - 1, cur.1),
                default => panic!("Non direction!")
            };
            
            if !h.insert(cur) {
                return true;
            }
        }

        false
    }
}
