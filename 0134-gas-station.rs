impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut cur_gas = 0;
        let mut lowest_index = 0;
        let mut lowest_gas = i32::MAX;
        for i in 0..gas.len() {
            cur_gas += gas[i] - cost[i];
            if cur_gas < lowest_gas {
                lowest_index = i as i32;
                lowest_gas = cur_gas;
            }
        }
        let adjusted_index = lowest_index + 1;

        if cur_gas < 0 {
            -1
        }
        else if adjusted_index < gas.len() as i32 {
            adjusted_index
        }
        else {
            0
        }
    }
}
