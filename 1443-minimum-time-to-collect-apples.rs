use std::collections::HashMap;

impl Solution {
    pub fn min_time(n: i32, mut edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        // Create vector graph
        let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize];
        for i in 0..edges.len() {
            let (s, t) = (edges[i][0], edges[i][1]);
            graph[s as usize].push(t);
            graph[t as usize].push(s);
        }

        // Do traversal
        Self::dfs(&graph, &has_apple, 0, -1)        
    }

    fn dfs(graph: &Vec<Vec<i32>>, has_apple: &Vec<bool>, node: i32, prev: i32) -> i32 {
        let mut result = 0;
        for cur_node in &graph[node as usize] {
            let cur_node = *cur_node;
            if cur_node == prev {
                continue;
            }
            let dfs_res = Self::dfs(graph, has_apple, cur_node, node);

            if(dfs_res > 0 || has_apple[cur_node as usize]) {
                result += dfs_res + 2;
            }
        }

        result
    }
}

