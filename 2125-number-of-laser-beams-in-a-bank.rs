impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut prev_num_devices = 0;
        let mut total_beams = 0;
        for row in bank {
            let cur_num_devices = row.as_bytes().iter().filter(|x| **x == b'1').count();
            if cur_num_devices == 0 {
                continue;
            }
            
            total_beams += prev_num_devices * cur_num_devices;
            prev_num_devices = cur_num_devices;
        }
        total_beams as i32
    }
}
