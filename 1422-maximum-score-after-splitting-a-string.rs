impl Solution {
    pub fn max_score(s: String) -> i32 {
        let last_index = s.len() - 1;
        let mut from_front = Self::count_char('0', s.chars(), last_index);
        let mut from_back = Self::count_char('1', s.chars().rev(), last_index);
        from_back.iter().zip(from_front.iter().rev()).map(|t| t.0 + t.1).max().unwrap()
    }

    fn count_char<I>(token: char, iter: I, last_index: usize) -> Vec<i32> where I: Iterator<Item = char> {
        let mut vector = vec![0; last_index];

        let mut place = 0;
        for c in iter {
            if place == last_index {
                break;
            }

            let prev = if place > 0 { vector[place - 1] } 
            else { 0 };

            if c == token { 
                vector[place] = 1 + prev;
            }
            else {
                vector[place] = prev;
            }
            place += 1;
        }

        vector
    }
}
