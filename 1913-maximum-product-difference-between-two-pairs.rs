impl Solution {
  // Aware that I could use sort_unstable, but that would be slower
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        // Find the minimum 2 and the maximum 2
        let (mut min_1, mut min_2) = (i32::MAX, i32::MAX);
        let (mut max_1, mut max_2) = (i32::MIN, i32::MIN);

        for n in nums.iter() {
            let n = *n;
            if n > max_1 {
                max_2 = max_1;
                max_1 = n;
            }
            else if n > max_2 {
                max_2 = n;
            }

            if n < min_1 {
                min_2 = min_1;
                min_1 = n;
            }
            else if n < min_2 {
                min_2 = n;
            }
        }

        (max_1 * max_2) - (min_1 * min_2)
    }
}
