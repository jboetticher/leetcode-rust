impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, coins: i32) -> i32 {
        costs.sort_unstable();
        let mut sum = 0;
        for i in 0..costs.len() {
            sum += costs[i];
            if sum > coins {
                return i as i32;
            }
        }

        costs.len() as i32
    }
}
