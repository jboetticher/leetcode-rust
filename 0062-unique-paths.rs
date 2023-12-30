impl Solution {
    // TODO: optimize memory
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // Apparently m is y and n is x
        let (m, n) = (m as usize, n as usize);
        let mut memo = vec![vec![0; n]; m];   

        // There is only ever one path on the first column and row. 
        // (if we always go left or always go down)
        for i in 0..m { memo[i][0] = 1; }
        for i in 0..n { memo[0][i] = 1; }

        // We now have an array like so:
        //  1   1   1   ...
        //  1   0   0   ...
        //  1   0   0   ...
        //  ... ... ... ...

        // There are two possible steps at all times that a robot could take.
        // 1. Move to the right
        // 2. Move to the bottom
        // So, we can move through the array to the right and to the bottom
        for y in 1..m {                                 
            for x in 1..n {
                memo[y][x] = memo[y - 1][x] + memo[y][x-1];
            }
        }

        memo[m - 1][n - 1] as i32
    }
}
