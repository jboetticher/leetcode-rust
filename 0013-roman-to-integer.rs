impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut cIter = chars.iter();

        let mut total = 0;
        let mut prev = 'x';
        while let Some(c) = cIter.next() {
            total += match c {
                'I' => 1,
                'V' => Solution::pref_roman_mod(&prev, 'I', 1, 5), 
                'X' => Solution::pref_roman_mod(&prev, 'I', 1, 10),
                'L' => Solution::pref_roman_mod(&prev, 'X', 10, 50),
                'C' => Solution::pref_roman_mod(&prev, 'X', 10, 100),
                'D' => Solution::pref_roman_mod(&prev, 'C', 100, 500),
                'M' => Solution::pref_roman_mod(&prev, 'C', 100, 1000),
                _ => 0
            };
            prev = *c;
        }
        total
    }

    fn pref_roman_mod(prev: &char, mod_char: char, prev_val: i32, default_val: i32) -> i32 {
        if *prev == mod_char {
            default_val - 2 * prev_val
        }
        else {
            default_val
        }
    }
}
