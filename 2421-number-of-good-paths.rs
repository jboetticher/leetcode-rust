// Another union find >:[
// Also "inspired" by Leetcode the Hard Way; but with better explanations
use std::cmp::max;

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        // Union Find (DSU)
        let n = vals.len();
        let mut roots = (0..n).collect();
        fn find(x: usize, root: &Vec<usize>) -> usize {
            match root[x] == x {
                true => x,
                false => find(root[x], root)
            }
        }

        // Sort edges by the largest node in each edge, to help satisfy the <= requirement
        let mut edges = edges;
        edges.sort_by(
            |x, y| max(vals[x[0] as usize], vals[x[1] as usize]).cmp(&max(vals[y[0] as usize], vals[y[1] as usize]))
        );

        // Create a count vector to keep track of the number of paths per node
        let mut count = vec![1; n];

        // Iterate with union
        edges.iter().fold(n as i32, |ans, edge| {
            let mut ans: i32 = ans;

            // Get the root of x
            let x = find(edge[0] as usize, &roots);
            // Get the root of y
            let y = find(edge[1] as usize, &roots);

            // If their values are the same...
            if vals[x] == vals[y] {
                // then there would be cnt[x] * cnt[y] good paths
                ans += count[x] as i32 * count[y] as i32;

                // unite them
                roots[x] = y;

                // add the count of x to that of y
                count[y] += count[x];
            }
            else if vals[x] > vals[y] {
                // unite them
                roots[y] = x;
            } else {
                // unite them
                roots[x] = y;
            }

            ans
        })
    }
}
