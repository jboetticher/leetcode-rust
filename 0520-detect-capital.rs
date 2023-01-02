use std::collections::VecDeque;

// Suboptimal solution but I've never used VecDeque and I wanted to try it out
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut chars: VecDeque<char> = word.chars().collect();

        let mut first_capital: bool = match chars.pop_front() {
            None => panic!("Words should be 1 character or more!"),
            Some(c) => c.is_uppercase()
        };
        let mut all_capital: bool = first_capital;
        let mut all_lower: bool = !first_capital;

        for c in chars {
            all_capital = all_capital && c.is_uppercase();
            all_lower = all_lower && c.is_lowercase();
            first_capital = first_capital && c.is_lowercase();

            if !all_capital && !all_lower && !first_capital {
                return false;
            }
        }

        all_capital || all_lower || first_capital
    }
}
