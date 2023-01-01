use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.as_str().split(" ").collect();

        let mut pattern_map: HashMap<char, String> = HashMap::new();
        let mut added_words: Vec<String> = vec![];
        let pattern: Vec<char> = pattern.chars().collect();
        let mut place = 0;
        let mut follows_pattern = true;

        if words.len() != pattern.len() {
            return false;
        }

        for w in words {
            match pattern_map.get(&pattern[place]) {
                None => { 
                    let w = w.to_string();
                    if added_words.contains(&w) {
                        follows_pattern = false;
                    }
                    else {
                        pattern_map.insert(pattern[place], w.clone()); 
                        added_words.push(w);
                    }
                },
                Some(res_w) => {
                    let res_w = res_w.as_str();
                    if res_w != w {
                        follows_pattern = false;
                    }
                }
            }

            if !follows_pattern {
                return false;
            }

            place += 1;
        }

        follows_pattern
    }
}
