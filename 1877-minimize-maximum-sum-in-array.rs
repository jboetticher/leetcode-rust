impl Solution {
    // Learned about .zip today, which can iterate between two iterators at the same time! wow
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        nums.sort_unstable();
        nums[..len/2]
            .iter()
            .zip(nums[len/2..].iter().rev())
            .map(|(a,b)| a+b)
            .max()
            .unwrap()
    }

    // My original, slightly inefficient solution
    
    // pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    //     let len = nums.len();
    //     nums.sort_unstable();
    //     let mut max = 0;
    //     for i in 0..(len / 2) {
    //         let sum = nums[i] + nums[len - i - 1];
    //         if sum > max { max = sum; }
    //     }
    //     max
    // }
}
