use std::cmp;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut largest = cmp::max(nums[0], nums[1]);
        let mut second_largest = cmp::min(nums[0], nums[1]);
        if nums.len() > 2 {
            for i in 2..nums.len() {
                if nums[i] > largest {
                    second_largest = largest;
                    largest = nums[i];
                }
                else if nums[i] > second_largest {
                    second_largest = nums[i];
                }
            }
        }
        (largest - 1) * (second_largest - 1)
    }
}
