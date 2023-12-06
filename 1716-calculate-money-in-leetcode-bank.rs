impl Solution {
    // 1 + 2 + 3 + 4 + 5 + 6 + 7        28
    // 2 + 3 + 4 + 5 + 6 + 7 + 8        28 + 7
    // 3 + 4 + 5 + 6 + 7 + 8 + 9        28 + 7 + 7
    // 4 + 5 + 6 + 7 + 8 + 9 + 10       28 + 7 + 7 + 7
    pub fn total_money(n: i32) -> i32 {
        let left_over = n % 7;
        let total_weeks = n / 7;

        // 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28
        let mut total = total_weeks * 28; 

        // Apply Gauss's consecutive sum formula for consecutive full weeks
        if total_weeks > 1 {
            total += 7 * (total_weeks - 1) * (1 + (total_weeks - 1)) / 2;
        } 

        // Apply Gauss's consecutive sum formula
        if left_over > 0 {
            total += left_over * ((total_weeks + 1) + (total_weeks + left_over)) / 2;
        }

        total
    }
}
