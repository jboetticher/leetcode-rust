use std::collections::HashMap;

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let twenty_five = arr.len() / 4;
        let (mut cur, mut cur_count) = (arr[0], 0);

        for i in 0..arr.len() {
            if cur == arr[i] { cur_count += 1; }
            else { 
                cur = arr[i];
                cur_count = 1;
            }
            if cur_count > twenty_five { return cur; }
        }

        return -1;
    }
}
