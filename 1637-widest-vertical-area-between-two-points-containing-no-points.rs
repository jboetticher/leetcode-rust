use std::collections::BTreeSet;

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let ordered_set: BTreeSet<i32> = points.iter().map(|x| x[0]).collect();
        ordered_set.iter().fold((ordered_set.first().unwrap(), 0), |acc, x| {
            if (x - acc.0 > acc.1) {
                (x, x - acc.0)
            }
            else {
                (x, acc.1)
            }
        }).1
    }
}
