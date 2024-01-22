// This solution used a HashSet because I didn't realize the numbers had to be in order
use std::collections::HashSet;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        // MISSING = (n(n+1)/2) + DUPLICATE - SUM
        let (mut duplicate, mut sum, mut set) = (0, 0, HashSet::<i32>::new());
        for v in nums.iter() { 
            sum += v;
            if !set.insert(*v) {
                duplicate = *v;
            }
        }

        let original_sum = (nums.len() * (nums.len() + 1) / 2) as i32;
        vec![duplicate, original_sum + duplicate - sum]
    }
}
