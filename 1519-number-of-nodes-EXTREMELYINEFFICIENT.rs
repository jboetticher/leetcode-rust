// Really not proud of this one. Don't consider it a solution, please.
impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let labels: Vec<char> = labels.chars().collect();
        let mut connections: Vec<Vec<i32>> = vec![vec![]; n as usize];
        for i in 0..edges.len() {
            let (node1, node2) = (edges[i][0], edges[i][1]); 
            connections[node1 as usize].push(node2);
            connections[node2 as usize].push(node1);
        }

        let mut answers: Vec<i32> = vec![1; n as usize];
        let mut finished: Vec<bool> = vec![false; n as usize];
        answers[0] += Self::dfs(0, -1, &labels[0], &connections, &labels, &mut answers, &mut finished);
        
        answers
    }

    // dfs to just find the root node. How do we expand this to work for multiple nodes?
    // Or... maybe we could just do a loop of dfs? No, that wouldn't work because it's undirected
    fn dfs(node: i32, prev: i32, selector: &char, connections: &Vec<Vec<i32>>, 
        labels: &Vec<char>, answers: &mut Vec<i32>, finished: &mut Vec<bool>) -> i32 {
        let mut result = 0;
        for cur_node in &connections[node as usize] {
            let cur_node = *cur_node;
            if cur_node == prev || cur_node == node {
                continue;
            }
            
            // Root node check
            if *selector == labels[cur_node as usize] {
                result += 1;
            }

            result += Self::dfs(cur_node, node, selector, connections, labels, answers, finished);
            if !finished[cur_node as usize] {
                finished[cur_node as usize] = true;
                answers[cur_node as usize] += 
                    Self::dfs(cur_node, node, &labels[cur_node as usize], connections, labels, answers, finished);
            }
        }

        result
    }
}
