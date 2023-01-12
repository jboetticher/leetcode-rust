// This solution was made by Minamikaze392, where I retrofitted my code to fit their solution. :(

impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let labels: &[u8] = labels.as_bytes();
        let mut connections: Vec<Vec<usize>> = vec![vec![]; n as usize];
        for i in 0..edges.len() {
            let (node1, node2) = (edges[i][0], edges[i][1]); 
            connections[node1 as usize].push(node2 as usize);
            connections[node2 as usize].push(node1 as usize);
        }

        let mut answers: Vec<i32> = vec![0; n as usize];
        Self::dfs(n as usize, 0, &mut answers, &connections, labels);
        
        answers
    }

    fn dfs(prev: usize, curr: usize, answers: &mut Vec<i32>, connections: &Vec<Vec<usize>>, bytes: &[u8]) -> [i32; 26] {
        let mut res = [0; 26];
        for i in &connections[curr] {
            let i = *i;
            if i != prev {
                let dfs_res = Self::dfs(curr, i, answers, connections, bytes);
                for j in 0..26 {
                    res[j] += dfs_res[j];
                }
            }
        }

        let bytes_index = (bytes[curr] - b'a') as usize;
        res[bytes_index] += 1;
        answers[curr] = res[bytes_index];

        res
    }
}
