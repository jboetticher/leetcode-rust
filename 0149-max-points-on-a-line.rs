use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            return 1;
        }

        // Seems like an O(n^2) problem?
        // For each pair of points, find the y = mx + b, and hash by (m, b)
        // b = y - mx
        let mut slope_counts = HashMap::<(i64, i64), i32>::new();

        for point in &points {
            for compare_point in &points {
                // Find the mx + b
                let m = match (compare_point[0] as f64 - point[0] as f64) {
                    0.0 => (f64::INFINITY),
                    d => (compare_point[1] as f64 - point[1] as f64) / d
                };
                let b = match m {
                    f64::INFINITY => point[0] as f64,
                    m => point[1] as f64 - m * point[0] as f64
                };

                slope_counts
                    .entry((Solution::integerize(m), Solution::integerize(b)))
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }

        match slope_counts.values().max() {
            None => 1,
            Some(i) => {
                println!("Got to this point {}", *i);
                // y = x^2 + x - *i
                let delta = 1 + 4 * *i;
                let sol1 = (f64::sqrt(delta as f64) as i32 - 1) / 2;
                return sol1 + 1;
            }
        }
    }

    fn integerize(f: f64) -> i64 {
        (f * 1_000_000_000_000.0).round() as i64
    }
}
