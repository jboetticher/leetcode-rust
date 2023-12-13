impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat[0].len();
        let mut row_count = vec![0; mat.len()];
        let mut column_count = vec![0; n];

        for r in 0..mat.len() {
            for c in 0..n {
                row_count[r] += mat[r][c];
                column_count[c] += mat[r][c];
            }
        }

        let mut special = 0;
        'row_loop: for mut r in 0..mat.len() {
            for mut c in 0..n {
                if mat[r][c] == 1 && row_count[r] == 1 && column_count[c] == 1 {
                    special += 1;
                    continue 'row_loop;
                }
            }
        }

        special
    }
}
