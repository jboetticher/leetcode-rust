use std::collections::HashMap;

impl Solution {
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut difficulty_count = HashMap::<i32, i32>::new();
        for i in tasks {
            let cur_v = difficulty_count.get(&i);
            match cur_v {
                None => difficulty_count.insert(i, 1),
                Some(v) => difficulty_count.insert(i, v + 1)
            };
        }

        let mut rounds = 0;
        for n in difficulty_count.keys() {
            let r = difficulty_count[n] % 3;
            if difficulty_count[n] == 1  {
                return -1;
            }
            else if r == 2 || r == 1 {
                rounds += 1;
            }
            rounds += difficulty_count[n] / 3;
        }

        rounds
    }
}
