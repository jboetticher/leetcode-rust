impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut character_indicies = vec![vec![]; 26];

        let mut index = 0;
        for c in s.as_bytes() {
            character_indicies[(c - 'a' as u8) as usize].push(index);
            index += 1;
        }

        let mut max = 0;
        for indicies in character_indicies {
            let len = indicies.len();
            if len < 2 { continue; }
            let largest_dif = indicies[len - 1] - indicies[0];
            if largest_dif > max { max = largest_dif; }
        }

        max - 1
    }
}
