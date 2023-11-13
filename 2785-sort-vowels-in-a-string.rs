impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut indexes: Vec<usize> = vec![];

        // 1. Get a list of the vowels
        let mut vowels: Vec<char> = s.chars()
            .filter(|c| Solution::is_vowel(*c))
            .collect();

        // 2. Sort the vowels
        vowels.sort_unstable_by(|a, b| b.cmp(a));

        // 3. Replace at each vowel
        let mut result = String::new();
        s.chars().for_each(|c| 
            if Solution::is_vowel(c) { 
                result.push(vowels.pop().unwrap()); 
            } 
            else {
                result.push(c);
            }
        );

        result
    }

    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' ||
        c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U'
    }
}

/*
This was my first solution, which was very inefficient because of the reuse of iterators.
Still faster than Python.

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut vowels: Vec<char> = vec![];
        let mut indexes: Vec<usize> = vec![];

        // 1. Get a list of the indexes that have vowels & get a list of the vowels
        let mut i = 0;
        for c in s.chars() {
            if Solution::is_vowel(c) {
                vowels.push(c);
                indexes.push(i);
            }
            i += 1;
        }

        // 2. Sort the vowels
        vowels.sort();

        // 3. Replace at each index
        let mut i = 0;
        let mut chars: Vec<char> = s.chars().collect();
        for index in indexes {
            chars[index] = vowels[i];
            i += 1;
        }

        chars.iter().collect()
    }

    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' ||
        c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U'
    }
}

*/
