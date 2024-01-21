impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 { return nums[0]; }

        let mut dp: [i32; 2] = [nums[0], nums[0].max(nums[1])];
        for i in 2..n {
            let cur_v = dp[1].max(dp[0] + nums[i]);
            dp[0] = dp[1];
            dp[1] = cur_v;
        }

        dp[1]
    }
}
