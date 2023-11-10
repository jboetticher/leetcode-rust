// Beats 100% in both memory and runtime. I feel like a chadlord.

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        // Iterate through the strings, and sum up the contiguous characters
        let mut homo_sum: i128 = 0;
        let mut cur_count: i128 = 0;
        let mut cur_char = 'A';
        for s in s.chars() {
            if cur_char != s {
                homo_sum += (cur_count * (cur_count + 1)) / 2;
                cur_char = s;
                cur_count = 1;
                continue;
            }
            
            cur_count += 1;
        }
        homo_sum += (cur_count * (cur_count + 1)) / 2;

        (homo_sum % (1_000_000_000 + 7)) as i32
    }
}
