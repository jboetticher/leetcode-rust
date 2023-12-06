impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut open_brackets = vec![];
        for c in s.chars() {
            if !match c {
                '(' | '{' | '[' => {
                    open_brackets.push(c);
                    true
                },
                '}' | ')' | ']' => {
                    let alt_c = open_brackets.pop();
                    match alt_c {
                        Some(alt_c) => {
                            // This calculation is inelegant due to how the () ASCII pair is different from {} & []
                            if alt_c as u32 + 1 != c as u32 && alt_c as u32 + 2 != c as u32 {
                                return false;
                            }
                            true
                        }
                        None => false
                    }
                },
                default => panic!("INVALID INPUT")
            } {
                return false;
            }
        }

        open_brackets.len() == 0
    }
}
