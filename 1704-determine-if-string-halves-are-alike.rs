impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let middle = s.len() / 2;
        Self::number_of_vowels(&s[..middle]) == 
            Self::number_of_vowels(&s[middle..])
    }

    pub fn number_of_vowels(s: &str) -> i32 {
        s.chars().fold(0, |acc, c| { 
            if "aAeEiIoOuU".chars().any(|x| x == c) { acc + 1 }
            else { acc }
        })
    }
}
