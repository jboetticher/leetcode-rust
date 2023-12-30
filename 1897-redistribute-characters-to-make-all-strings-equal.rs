impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut counts = [0; 26];
        words
            .iter()
            .flat_map(|w| w.as_bytes())
            .for_each(|c| counts[(c - 'a' as u8) as usize] += 1 );
        counts.iter().all(|x| x % words.len() == 0)
    }
}
