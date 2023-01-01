use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_to_index = HashMap::<i32, i32>::new();
        let mut answer: Vec<i32> = vec![];

        for i in 0..nums.len() {
            let dif: i32 = target - nums[i];
            if match num_to_index.get(&dif) {
                None => {
                    num_to_index.insert(nums[i], i as i32);
                    false
                },
                Some(val) => {
                    answer.push(*val);
                    answer.push(i as i32);
                    true
                }
            } {
                break;
            };
        }

        answer
    }
}
