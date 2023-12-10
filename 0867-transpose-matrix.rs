impl Solution {
    pub fn transpose(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut answer: Vec<Vec<i32>> = vec![];

        // Initialize answer array
        for i in 0..n {
            answer.push(vec![]);
        }

        // Transpose
        for i in 0..m {
            for k in 0..n {
                answer[k].push(matrix[i][k]);
            }
        }

        answer
    }
}
