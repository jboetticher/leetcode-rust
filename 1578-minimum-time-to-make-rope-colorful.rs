impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut prev_not_removed_index = 0;
        let mut time = 0;

        let c = colors.as_bytes();

        for i in 1..needed_time.len() {
            if c[prev_not_removed_index] == c[i] {
                if needed_time[prev_not_removed_index] > needed_time[i] {
                    time += needed_time[i];
                }
                else {
                    time += needed_time[prev_not_removed_index];
                    prev_not_removed_index = i;
                }
            }
            else {
                prev_not_removed_index = i;
            }
        }

        time
    }
}
