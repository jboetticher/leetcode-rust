use std::{cmp::min, mem::swap};
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let (mut prev_dp, mut cur_dp) = (vec![0; n], vec![0; n]);
        for i in 0..n { prev_dp[i] = matrix[0][i]; }

        // dp represents the minimum falling sum to get to that location
        for i in 1..n {
            for k in 0..n {
                if (k == 0) {
                    cur_dp[k] = matrix[i][k] + min(prev_dp[0], prev_dp[1]);
                }
                else if (k == n - 1) {
                    cur_dp[k] = matrix[i][k] + min(prev_dp[k - 1], prev_dp[k]);
                }
                else {
                    cur_dp[k] = matrix[i][k] + min(min(prev_dp[k - 1], prev_dp[k]), prev_dp[k + 1]);
                }
            }

            // Then swap between prev_dp & cur_dp
            swap(&mut prev_dp, &mut cur_dp);
        }

        // Find minimum of prev_dp
        *prev_dp.iter().min().unwrap()
    }
}
